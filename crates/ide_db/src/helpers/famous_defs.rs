//! See [`FamousDefs`].
use hir::{Crate, Enum, MacroDef, Module, ScopeDef, Semantics, Trait};

use crate::IdeDatabase;

/// Helps with finding well-know things inside the standard library. This is
/// somewhat similar to the known paths infra inside hir, but it different; We
/// want to make sure that IDE specific paths don't become interesting inside
/// the compiler itself as well.
///
/// Note that, by default, husky-lang-server tests **do not** include core or std
/// libraries. If you are writing tests for functionality using [`FamousDefs`],
/// you'd want to include minicore (see `test_utils::MiniCore`) declaration at
/// the start of your tests:
///
/// ```
/// //- minicore: iterator, ord, derive
/// ```
pub struct FamousDefs<'a, 'b>(pub &'a Semantics<'b, IdeDatabase>, pub Option<Crate>);

#[allow(non_snake_case)]
impl FamousDefs<'_, '_> {
    pub fn std(&self) -> Option<Crate> {
        self.find_crate("std")
    }

    pub fn core(&self) -> Option<Crate> {
        self.find_crate("core")
    }

    pub fn core_cmp_Ord(&self) -> Option<Trait> {
        self.find_trait("core:cmp:Ord")
    }

    pub fn core_convert_From(&self) -> Option<Trait> {
        self.find_trait("core:convert:From")
    }

    pub fn core_convert_Into(&self) -> Option<Trait> {
        self.find_trait("core:convert:Into")
    }

    pub fn core_option_Option(&self) -> Option<Enum> {
        self.find_enum("core:option:Option")
    }

    pub fn core_result_Result(&self) -> Option<Enum> {
        self.find_enum("core:result:Result")
    }

    pub fn core_default_Default(&self) -> Option<Trait> {
        self.find_trait("core:default:Default")
    }

    pub fn core_iter_Iterator(&self) -> Option<Trait> {
        self.find_trait("core:iter:traits:iterator:Iterator")
    }

    pub fn core_iter_IntoIterator(&self) -> Option<Trait> {
        self.find_trait("core:iter:traits:collect:IntoIterator")
    }

    pub fn core_iter(&self) -> Option<Module> {
        self.find_module("core:iter")
    }

    pub fn core_ops_Deref(&self) -> Option<Trait> {
        self.find_trait("core:ops:Deref")
    }

    pub fn core_convert_AsRef(&self) -> Option<Trait> {
        self.find_trait("core:convert:AsRef")
    }

    pub fn core_ops_ControlFlow(&self) -> Option<Enum> {
        self.find_enum("core:ops:ControlFlow")
    }

    pub fn core_marker_Copy(&self) -> Option<Trait> {
        self.find_trait("core:marker:Copy")
    }

    pub fn core_macros_builtin_derive(&self) -> Option<MacroDef> {
        self.find_macro("core:macros:builtin:derive")
    }

    pub fn alloc(&self) -> Option<Crate> {
        self.find_crate("alloc")
    }

    pub fn test(&self) -> Option<Crate> {
        self.find_crate("test")
    }

    pub fn proc_macro(&self) -> Option<Crate> {
        self.find_crate("proc_macro")
    }

    pub fn builtin_crates(&self) -> impl Iterator<Item = Crate> {
        IntoIterator::into_iter([
            self.std(),
            self.core(),
            self.alloc(),
            self.test(),
            self.proc_macro(),
        ])
        .filter_map(|it| it)
    }

    fn find_trait(&self, path: &str) -> Option<Trait> {
        todo!()
    }

    fn find_macro(&self, path: &str) -> Option<MacroDef> {
        todo!()
    }

    fn find_enum(&self, path: &str) -> Option<Enum> {
        todo!()
    }

    fn find_module(&self, path: &str) -> Option<Module> {
        todo!()
    }

    fn find_crate(&self, name: &str) -> Option<Crate> {
        todo!()
    }

    fn find_def(&self, path: &str) -> Option<ScopeDef> {
        todo!()
    }
}
