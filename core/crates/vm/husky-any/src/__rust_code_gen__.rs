// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;

// VirtualStruct
#[no_mangle]
pub unsafe extern "C" fn __virtual_struct_clone(data: *mut ()) -> *mut () {
    Box::<VirtualStruct>::into_raw(Box::new((*(data as *mut VirtualStruct)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_struct_drop(data: *mut ()) {
    Box::from_raw(data as *mut VirtualStruct);
}
extern "C" {
    pub static __VIRTUAL_STRUCT_VTABLE: __RegisterVTable;
}

// VirtualVec
#[no_mangle]
pub unsafe extern "C" fn __virtual_vec_clone(data: *mut ()) -> *mut () {
    Box::<VirtualVec>::into_raw(Box::new((*(data as *mut VirtualVec)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_vec_drop(data: *mut ()) {
    Box::from_raw(data as *mut VirtualVec);
}
extern "C" {
    pub static __VIRTUAL_VEC_VTABLE: __RegisterVTable;
}

// VirtualCyclicSlice
#[no_mangle]
pub unsafe extern "C" fn __virtual_cyclic_slice_clone(data: *mut ()) -> *mut () {
    Box::<VirtualCyclicSlice>::into_raw(Box::new((*(data as *mut VirtualCyclicSlice)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_cyclic_slice_drop(data: *mut ()) {
    Box::from_raw(data as *mut VirtualCyclicSlice);
}
extern "C" {
    pub static __VIRTUAL_CYCLIC_SLICE_VTABLE: __RegisterVTable;
}
