use crate::win::{self, HMODULE};
use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};

pub type Ptr = *mut c_void;

pub struct Mono {
    domain_assembly_open: unsafe extern "C" fn(Ptr, *const c_char) -> Ptr,
    assembly_get_image: unsafe extern "C" fn(Ptr) -> Ptr,
    class_from_name: unsafe extern "C" fn(Ptr, *const c_char, *const c_char) -> Ptr,
    class_get_method_from_name: unsafe extern "C" fn(Ptr, *const c_char, c_int) -> Ptr,
    runtime_invoke: unsafe extern "C" fn(Ptr, Ptr, *mut Ptr, *mut Ptr) -> Ptr,
    object_to_string: unsafe extern "C" fn(Ptr, *mut Ptr) -> Ptr,
    string_to_utf8: unsafe extern "C" fn(Ptr) -> *mut c_char,
    free: unsafe extern "C" fn(Ptr),
    add_internal_call: unsafe extern "C" fn(*const c_char, *const c_void),
    string_chars: unsafe extern "C" fn(Ptr) -> *const u16,
    string_length: unsafe extern "C" fn(Ptr) -> c_int,
    thread_current: Option<unsafe extern "C" fn() -> Ptr>,
    thread_set_main: Option<unsafe extern "C" fn(Ptr)>,
}

macro_rules! sym {
    ($m:expr, $name:literal) => {{
        let p = unsafe { win::GetProcAddress($m, concat!($name, "\0").as_ptr()) };
        if p.is_null() { None } else { Some(unsafe { core::mem::transmute(p) }) }
    }};
}

impl Mono {
    pub fn load(m: HMODULE) -> Result<Self, &'static str> {
        macro_rules! req {
            ($name:literal) => {
                sym!(m, $name).ok_or($name)?
            };
        }
        Ok(Self {
            domain_assembly_open: req!("mono_domain_assembly_open"),
            assembly_get_image: req!("mono_assembly_get_image"),
            class_from_name: req!("mono_class_from_name"),
            class_get_method_from_name: req!("mono_class_get_method_from_name"),
            runtime_invoke: req!("mono_runtime_invoke"),
            object_to_string: req!("mono_object_to_string"),
            string_to_utf8: req!("mono_string_to_utf8"),
            free: req!("mono_free"),
            add_internal_call: req!("mono_add_internal_call"),
            string_chars: req!("mono_string_chars"),
            string_length: req!("mono_string_length"),
            thread_current: sym!(m, "mono_thread_current"),
            thread_set_main: sym!(m, "mono_thread_set_main"),
        })
    }

    pub fn add_icall(&self, name: &str, f: *const c_void) {
        let c = CString::new(name).unwrap();
        unsafe { (self.add_internal_call)(c.as_ptr(), f) };
    }

    pub unsafe fn string(&self, s: Ptr) -> String {
        if s.is_null() {
            return String::new();
        }
        let len = unsafe { (self.string_length)(s) } as usize;
        let p = unsafe { (self.string_chars)(s) };
        String::from_utf16_lossy(unsafe { core::slice::from_raw_parts(p, len) })
    }

    pub fn set_main_thread(&self) {
        if let (Some(cur), Some(set)) = (self.thread_current, self.thread_set_main) {
            unsafe { set(cur()) };
        }
    }

    pub fn invoke_static(&self, domain: Ptr, path: &str, ns: &str, class: &str, method: &str) -> Result<(), String> {
        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        let c_ns = CString::new(ns).unwrap();
        let c_class = CString::new(class).unwrap();
        let c_method = CString::new(method).unwrap();
        unsafe {
            let asm = (self.domain_assembly_open)(domain, c_path.as_ptr());
            if asm.is_null() {
                return Err(format!("cannot open assembly {path}"));
            }
            let image = (self.assembly_get_image)(asm);
            let klass = (self.class_from_name)(image, c_ns.as_ptr(), c_class.as_ptr());
            if klass.is_null() {
                return Err(format!("class {ns}.{class} not found"));
            }
            let m = (self.class_get_method_from_name)(klass, c_method.as_ptr(), 0);
            if m.is_null() {
                return Err(format!("method {ns}.{class}.{method}() not found"));
            }
            let mut exc: Ptr = core::ptr::null_mut();
            (self.runtime_invoke)(m, core::ptr::null_mut(), core::ptr::null_mut(), &mut exc);
            if !exc.is_null() {
                return Err(self.describe(exc));
            }
        }
        Ok(())
    }

    unsafe fn describe(&self, obj: Ptr) -> String {
        let mut exc: Ptr = core::ptr::null_mut();
        let s = unsafe { (self.object_to_string)(obj, &mut exc) };
        if s.is_null() || !exc.is_null() {
            return "managed exception (ToString failed)".into();
        }
        let p = unsafe { (self.string_to_utf8)(s) };
        if p.is_null() {
            return "managed exception".into();
        }
        let out = unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned();
        unsafe { (self.free)(p as Ptr) };
        out
    }
}
