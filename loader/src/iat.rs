use crate::win::{self, HMODULE};
use core::ffi::c_void;

unsafe fn rd<T: Copy>(addr: usize) -> T {
    unsafe { core::ptr::read_unaligned(addr as *const T) }
}

pub unsafe fn hook(module: HMODULE, dll: &str, func: &str, new: usize) -> Option<usize> {
    let base = module as usize;
    if base == 0 || unsafe { rd::<u16>(base) } != 0x5A4D {
        return None;
    }
    let nt = base + unsafe { rd::<i32>(base + 0x3C) } as usize;
    if unsafe { rd::<u32>(nt) } != 0x4550 {
        return None;
    }
    let opt = nt + 24;
    if unsafe { rd::<u16>(opt) } != 0x20B {
        return None;
    }
    let import_rva = unsafe { rd::<u32>(opt + 112 + 8) } as usize;
    if import_rva == 0 {
        return None;
    }
    let mut desc = base + import_rva;
    loop {
        let name_rva = unsafe { rd::<u32>(desc + 12) } as usize;
        if name_rva == 0 {
            return None;
        }
        let name = unsafe { win::cstr((base + name_rva) as *const u8) };
        if name.eq_ignore_ascii_case(dll.as_bytes()) {
            let oft = unsafe { rd::<u32>(desc) } as usize;
            let ft = unsafe { rd::<u32>(desc + 16) } as usize;
            if oft != 0 {
                let mut i = 0;
                loop {
                    let thunk = unsafe { rd::<u64>(base + oft + i * 8) };
                    if thunk == 0 {
                        break;
                    }
                    if thunk >> 63 == 0 {
                        let fname = unsafe { win::cstr((base + (thunk as usize & 0x7FFF_FFFF) + 2) as *const u8) };
                        if fname == func.as_bytes() {
                            return unsafe { patch((base + ft + i * 8) as *mut usize, new) };
                        }
                    }
                    i += 1;
                }
            }
        }
        desc += 20;
    }
}

unsafe fn patch(slot: *mut usize, new: usize) -> Option<usize> {
    let mut old_prot = 0u32;
    if unsafe { win::VirtualProtect(slot as *mut c_void, 8, win::PAGE_READWRITE, &mut old_prot) } == 0 {
        return None;
    }
    let old = unsafe { slot.replace(new) };
    unsafe { win::VirtualProtect(slot as *mut c_void, 8, old_prot, &mut old_prot) };
    Some(old)
}
