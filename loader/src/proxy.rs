use crate::win::{self, HMODULE};

macro_rules! proxy {
    ($($name:ident),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        mod slots {
            $(pub static mut $name: usize = 0;)*
        }

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            #[unsafe(naked)]
            pub unsafe extern "system" fn $name() {
                core::arch::naked_asm!("jmp qword ptr [rip + {0}]", sym slots::$name);
            }
        )*

        unsafe fn resolve(module: HMODULE) -> usize {
            let mut missing = 0;
            $(
                let p = unsafe { win::GetProcAddress(module, concat!(stringify!($name), "\0").as_ptr()) } as usize;
                if p == 0 {
                    missing += 1;
                }
                unsafe { slots::$name = p; }
            )*
            missing
        }
    };
}

proxy!(
    GetFileVersionInfoA,
    GetFileVersionInfoByHandle,
    GetFileVersionInfoExA,
    GetFileVersionInfoExW,
    GetFileVersionInfoSizeA,
    GetFileVersionInfoSizeExA,
    GetFileVersionInfoSizeExW,
    GetFileVersionInfoSizeW,
    GetFileVersionInfoW,
    VerFindFileA,
    VerFindFileW,
    VerInstallFileA,
    VerInstallFileW,
    VerLanguageNameA,
    VerLanguageNameW,
    VerQueryValueA,
    VerQueryValueW,
);

pub fn init() -> Result<(), String> {
    let path = format!("{}\\version.dll", win::system_dir());
    let module = unsafe { win::LoadLibraryW(win::wide(&path).as_ptr()) };
    if module.is_null() {
        return Err(format!("cannot load {path}"));
    }
    match unsafe { resolve(module) } {
        0 => Ok(()),
        n => Err(format!("{n} exports missing in {path}")),
    }
}
