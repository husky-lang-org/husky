use super::PRIMITIVE_TYPES;
use crate::NONPRIMITIVE_BUILTIN_TYPES;
use convert_case::{Case, Casing};
use husky_print_utils::p;
use std;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn write_rust_code(rust_path: &str) -> std::io::Result<()> {
    let mut buffer =
        File::create(rust_path).expect(&format!("rust path {rust_path} doesn't exist"));
    write!(
        buffer,
        r#"// this is generated by husky_vm_interface_codegen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;
"#
    );

    for ty in PRIMITIVE_TYPES {
        let uppercase_ty = ty.to_uppercase();
        let result = match *ty {
            "void" => "false",
            "bool" => "data",
            "i32" | "i64" | "b32" | "b64" => "data != 0",
            "f32" | "f64" => "data != 0.0",
            _ => panic!(),
        };
        write!(
            buffer,
            r#"
// {ty}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_value_to_bool(data: __RegisterData) -> bool {{
    let data = data.as_{ty};
    {result}
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_primitive_value_to_box(data: __RegisterData) -> *mut () {{
    let data = data.as_{ty};
    let ptr: *mut {ty} = Box::<{ty}>::into_raw(Box::new(data));
    ptr as *mut ()
}}
#[no_mangle]
pub unsafe extern "C" fn __{ty}_drop(data: *mut ()) {{
    Box::from_raw(data as *mut {ty});
}}
extern "C" {{
    pub static __{uppercase_ty}_VTABLE: __RegisterVTable;
}}
"#
        )?
    }

    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        let snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            buffer,
            r#"
// {ty}
#[no_mangle]
pub unsafe extern "C" fn __{snake_ty}_drop(data: *mut ()) {{
    Box::from_raw(data as *mut {ty});
}}
extern "C" {{
    pub static __{upper_snake_ty}_VTABLE: __RegisterVTable;
}}
"#
        )?
    }
    Ok(())
}
