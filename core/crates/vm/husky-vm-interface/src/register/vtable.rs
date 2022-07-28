use super::*;
use std::ffi::c_char;

// order matters!
#[repr(C)]
pub struct __RegisterVTable {
    pub typename_str: *const c_char,
    pub primitive_value_to_bool: Option<fn(data: __RegisterData) -> bool>,
    pub primitive_value_to_box: Option<fn(data: *mut __RegisterData) -> *mut ()>,
    pub clone: Option<fn(data: *mut ()) -> *mut ()>,
    pub drop: Option<fn(data: *mut ())>,
}

unsafe impl Sync for __RegisterVTable {}
unsafe impl Send for __RegisterVTable {}
