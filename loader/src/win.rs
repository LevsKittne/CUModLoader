#![allow(non_snake_case, clippy::upper_case_acronyms)]

use core::ffi::c_void;

pub type HMODULE = *mut c_void;

pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const PAGE_READWRITE: u32 = 0x04;

#[repr(C)]
#[derive(Default)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetProcAddress(module: HMODULE, name: *const u8) -> *mut c_void;
    pub fn LoadLibraryW(name: *const u16) -> HMODULE;
    pub fn GetModuleHandleW(name: *const u16) -> HMODULE;
    pub fn GetModuleFileNameW(module: HMODULE, buf: *mut u16, size: u32) -> u32;
    pub fn GetSystemDirectoryW(buf: *mut u16, size: u32) -> u32;
    pub fn VirtualProtect(addr: *mut c_void, size: usize, new: u32, old: *mut u32) -> i32;
    pub fn DisableThreadLibraryCalls(module: HMODULE) -> i32;
    pub fn AllocConsole() -> i32;
    pub fn SetConsoleOutputCP(cp: u32) -> i32;
    pub fn SetConsoleTitleW(title: *const u16) -> i32;
    pub fn GetCommandLineW() -> *const u16;
    pub fn GetLocalTime(st: *mut SYSTEMTIME);
    pub fn CreateMutexW(attrs: *mut c_void, owner: i32, name: *const u16) -> HANDLE;
    pub fn GetLastError() -> u32;
    pub fn CreateMutexA(attrs: *mut c_void, owner: i32, name: *const u8) -> HANDLE;
    pub fn GetCurrentProcessId() -> u32;
    pub fn CreateFileW(name: *const u16, access: u32, share: u32, sec: *mut c_void, disp: u32, flags: u32, template: *mut c_void) -> HANDLE;
    pub fn WriteConsoleW(h: HANDLE, buf: *const u16, len: u32, written: *mut u32, reserved: *mut c_void) -> i32;
    pub fn SetConsoleTextAttribute(h: HANDLE, attr: u16) -> i32;
    pub fn GetConsoleMode(h: HANDLE, mode: *mut u32) -> i32;
    pub fn SetConsoleMode(h: HANDLE, mode: u32) -> i32;
}

pub type HANDLE = isize;

pub const INVALID_HANDLE: HANDLE = -1;
pub const GENERIC_READ: u32 = 0x8000_0000;
pub const GENERIC_WRITE: u32 = 0x4000_0000;
pub const FILE_SHARE_RW: u32 = 0x3;
pub const OPEN_EXISTING: u32 = 3;
pub const ENABLE_QUICK_EDIT_MODE: u32 = 0x40;
pub const ENABLE_EXTENDED_FLAGS: u32 = 0x80;

pub fn open_console(name: &str, access: u32) -> HANDLE {
    unsafe {
        CreateFileW(wide(name).as_ptr(), access, FILE_SHARE_RW, core::ptr::null_mut(), OPEN_EXISTING, 0, core::ptr::null_mut())
    }
}

pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub unsafe fn from_wide(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut len = 0;
    while unsafe { *p.add(len) } != 0 {
        len += 1;
    }
    String::from_utf16_lossy(unsafe { core::slice::from_raw_parts(p, len) })
}

pub unsafe fn cstr<'a>(p: *const u8) -> &'a [u8] {
    let mut len = 0;
    while unsafe { *p.add(len) } != 0 {
        len += 1;
    }
    unsafe { core::slice::from_raw_parts(p, len) }
}

pub fn module_path(module: HMODULE) -> String {
    let mut buf = vec![0u16; 1024];
    let n = unsafe { GetModuleFileNameW(module, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    String::from_utf16_lossy(&buf[..n])
}

pub fn system_dir() -> String {
    let mut buf = vec![0u16; 512];
    let n = unsafe { GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) } as usize;
    String::from_utf16_lossy(&buf[..n])
}

pub fn instance_number() -> u32 {
    for n in 1..16 {
        let name = wide(&format!("Local\\CUModLoader.instance.{n}"));
        let h = unsafe { CreateMutexW(core::ptr::null_mut(), 0, name.as_ptr()) };
        if h != 0 && unsafe { GetLastError() } != 183 {
            return n;
        }
    }
    16
}
