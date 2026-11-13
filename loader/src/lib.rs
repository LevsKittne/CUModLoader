mod config;
mod iat;
mod log;
mod mono;
mod proxy;
mod win;

use core::ffi::{c_char, c_void};
use log::{error, info};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MANAGED_ENTRY: (&str, &str, &str) = ("ModLoader", "Entrypoint", "Start");

type JitInit = unsafe extern "C" fn(*const c_char, *const c_char) -> mono::Ptr;

static ORIG_JIT_INIT: AtomicUsize = AtomicUsize::new(0);
static MONO_MODULE: AtomicUsize = AtomicUsize::new(0);
static LOADER_DIR: OnceLock<PathBuf> = OnceLock::new();
static CONSOLE: OnceLock<bool> = OnceLock::new();
static MONO: OnceLock<mono::Mono> = OnceLock::new();

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(module: win::HMODULE, reason: u32, _: *mut c_void) -> i32 {
    if reason == win::DLL_PROCESS_ATTACH {
        unsafe { win::DisableThreadLibraryCalls(module) };
        if proxy::init().is_err() {
            return 0;
        }
        let _ = std::panic::catch_unwind(|| attach(module));
    }
    1
}

fn attach(module: win::HMODULE) {
    let self_path = PathBuf::from(win::module_path(module));
    let game_dir = self_path.parent().map(Path::to_path_buf).unwrap_or_default();
    let loader_dir = game_dir.join("ModLoader");
    if !loader_dir.is_dir() || !is_unity_player() {
        return;
    }
    let _ = std::fs::create_dir_all(loader_dir.join("mods"));
    let cfg = config::Config::load(&loader_dir.join("loader.ini"));
    let cmdline = unsafe { win::from_wide(win::GetCommandLineW()) };
    let enabled = cfg.flag("loader.enabled", true) && !cmdline.contains("--no-mods");
    let console = enabled && cfg.flag("loader.console", true);
    let instance = win::instance_number();
    let log_name = if instance == 1 { "log.txt".to_owned() } else { format!("log.{instance}.txt") };
    log::init(&loader_dir.join(log_name).to_string_lossy(), console);
    info!("CUModLoader {VERSION} attached (instance {instance}), game dir: {}", game_dir.display());
    if !enabled {
        info!("loader disabled, skipping");
        return;
    }
    let _ = CONSOLE.set(console);
    let _ = LOADER_DIR.set(loader_dir);

    let player = unsafe { win::GetModuleHandleW(win::wide("UnityPlayer.dll").as_ptr()) };
    let target = if player.is_null() { unsafe { win::GetModuleHandleW(core::ptr::null()) } } else { player };
    if cmdline.contains("--mp-multi-instance") {
        match unsafe { iat::hook(target, "kernel32.dll", "CreateMutexA", create_mutex_hook as *const () as usize) } {
            Some(_) => info!("multi-instance enabled"),
            None => error!("failed to hook CreateMutexA"),
        }
    }
    match unsafe { iat::hook(target, "kernel32.dll", "GetProcAddress", get_proc_address_hook as *const () as usize) } {
        Some(_) => info!("GetProcAddress hooked"),
        None => error!("failed to hook GetProcAddress in UnityPlayer.dll"),
    }
}

fn is_unity_player() -> bool {
    let exe = PathBuf::from(win::module_path(core::ptr::null_mut()));
    let Some(stem) = exe.file_stem() else { return false };
    exe.with_file_name(format!("{}_Data", stem.to_string_lossy())).is_dir()
}

unsafe extern "system" fn create_mutex_hook(attrs: *mut c_void, owner: i32, name: *const u8) -> win::HANDLE {
    if name.is_null() {
        return unsafe { win::CreateMutexA(attrs, owner, name) };
    }
    let mut unique = unsafe { win::cstr(name) }.to_vec();
    unique.extend_from_slice(format!(".{}\0", unsafe { win::GetCurrentProcessId() }).as_bytes());
    unsafe { win::CreateMutexA(attrs, owner, unique.as_ptr()) }
}

unsafe extern "system" fn get_proc_address_hook(module: win::HMODULE, name: *const u8) -> *mut c_void {
    let real = unsafe { win::GetProcAddress(module, name) };
    if real.is_null() || (name as usize) <= 0xFFFF {
        return real;
    }
    if unsafe { win::cstr(name) } == b"mono_jit_init_version" {
        ORIG_JIT_INIT.store(real as usize, Ordering::SeqCst);
        MONO_MODULE.store(module as usize, Ordering::SeqCst);
        return jit_init_hook as *mut c_void;
    }
    real
}

unsafe extern "C" fn jit_init_hook(root: *const c_char, runtime: *const c_char) -> mono::Ptr {
    if *CONSOLE.get().unwrap_or(&false) {
        log::enable_console();
    }
    let orig: JitInit = unsafe { core::mem::transmute(ORIG_JIT_INIT.load(Ordering::SeqCst)) };
    let domain = unsafe { orig(root, runtime) };
    if !domain.is_null() {
        let _ = std::panic::catch_unwind(|| boot(domain));
    }
    domain
}

fn boot(domain: mono::Ptr) {
    info!("mono initialized, root domain {domain:p}");
    let module = MONO_MODULE.load(Ordering::SeqCst) as win::HMODULE;
    let mono = match mono::Mono::load(module) {
        Ok(m) => MONO.get_or_init(|| m),
        Err(missing) => {
            error!("mono export missing: {missing}");
            return;
        }
    };
    mono.set_main_thread();
    mono.add_icall("ModLoader.Native::Log", icall_log as *const c_void);
    let Some(dir) = LOADER_DIR.get() else { return };
    let dll = dir.join("core").join("ModLoader.dll");
    if !dll.is_file() {
        error!("{} not found", dll.display());
        return;
    }
    let (ns, class, method) = MANAGED_ENTRY;
    match mono.invoke_static(domain, &dll.to_string_lossy(), ns, class, method) {
        Ok(()) => info!("managed loader started"),
        Err(e) => error!("managed loader failed: {e}"),
    }
}

unsafe extern "C" fn icall_log(level: i32, source: mono::Ptr, msg: mono::Ptr) {
    let Some(mono) = MONO.get() else { return };
    let _ = std::panic::catch_unwind(|| {
        let source = unsafe { mono.string(source) };
        let msg = unsafe { mono.string(msg) };
        log::write(log::Level::from_i32(level), &source, &msg);
    });
}
