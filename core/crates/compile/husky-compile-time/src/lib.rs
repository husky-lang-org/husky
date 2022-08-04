mod config;
mod impl_code_gen;
mod impl_diagnostics;
mod impl_load;
mod impl_necessary;
pub mod utils;

pub use config::*;
use entity_kind::TyKind;
pub use husky_ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use husky_diagnostics::DiagnosticQuery;
pub use husky_entity_route::{EntityRoute, InternEntityRoute};
pub use husky_entity_semantics::EntityDefnQueryGroup;
pub use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxSalsaQueryGroup};
pub use husky_file::{AllocateUniqueFile, FileQueryGroup, FileSalsaQuery, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use husky_infer_entity_route::*;
pub use husky_infer_qualified_ty::*;
pub use husky_linkage_table::ResolveLinkage;
pub use husky_package_semantics::PackageQueryGroup;
pub use husky_rust_code_gen::RustCodeGenQueryGroup;
pub use husky_token::TokenQueryGroup;
pub use husky_token::TokenSalsaQueryGroup;
pub use husky_word::InternWord;
pub use infer_contract::*;
pub use infer_decl::*;
pub use infer_total::*;

use husky_check_utils::*;
use husky_entity_route::{new_ty_route_cache, EntityRoutePtr};
use husky_entity_semantics::EntityRouteStore;
use husky_file::FilePtr;
use husky_linkage_table::LinkageTable;
use husky_print_utils::*;
use husky_word::RootIdentifier;
use indexmap::IndexMap;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};
use sync_utils::ASafeRwLock;
use vm::{__Register, __RegisterDataKind, __RegisterTyVTable, __I32_VTABLE};

#[salsa::database(
    husky_file::FileQueryStorage,
    husky_token::TokenQueryGroupStorage,
    husky_entity_syntax::ScopeQueryGroupStorage,
    husky_text::TextQueryGroupStorage,
    husky_ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    infer_decl::DeclQueryGroupStorage,
    husky_infer_entity_route::InferEntityRouteQueryGroupStorage,
    infer_contract::InferContractQueryGroupStorage,
    husky_infer_qualified_ty::InferQualifiedTyQueryGroupStorage,
    husky_entity_semantics::EntityQueryGroupStorage,
    husky_package_semantics::PackageQueryGroupStorage,
    husky_diagnostics::DiagnosticQueryGroupStorage,
    husky_rust_code_gen::RustGenQueryStorage
)]
pub struct HuskyComptime {
    storage: salsa::Storage<HuskyComptime>,
    file_interner: Arc<husky_file::FileInternerSingletonKeeper>,
    ty_cache: Arc<husky_entity_route::TyRouteCacheSingletonKeeper>,
    word_interner: Arc<husky_word::WordInternerSingletonKeeper>,
    entity_route_interner: Arc<husky_entity_route::EntityRouteInternerSingletonKeeper>,
    entity_route_menu: Arc<husky_entity_route::EntityRouteMenuSingletonKeeper>,
    live_docs: ASafeRwLock<IndexMap<FilePtr, ASafeRwLock<String>>>,
    linkage_table: LinkageTable,
    entity_route_store: EntityRouteStore,
    opt_main: Option<FilePtr>,
    config: HuskyCompileTimeConfig,
}

impl HuskyComptime {
    pub fn new(config: HuskyCompileTimeConfig) -> Self {
        let live_docs = Default::default();
        let entity_route_interner = husky_entity_route::new_entity_route_interner();
        let entity_route_store = Default::default();
        let linkage_table = LinkageTable::new(config.linkage_table.clone());
        Self {
            storage: Default::default(),
            file_interner: husky_file::new_file_interner(),
            word_interner: husky_word::new_word_interner(),
            entity_route_interner,
            live_docs,
            linkage_table,
            entity_route_store,
            opt_main: None,
            config,
            ty_cache: new_ty_route_cache(),
            entity_route_menu: husky_entity_route::new_entity_route_menu(),
        }
    }

    pub fn new_default(
        __root_defn: fn(
            ident: husky_word::RootIdentifier,
        ) -> &'static static_defn::EntityStaticDefn,
    ) -> Self {
        Self::new(HuskyCompileTimeConfig {
            package_dir: Default::default(),
            __resolve_root_defn: __root_defn,
            linkage_table: Default::default(),
        })
    }

    pub fn main_file(&self) -> FilePtr {
        self.opt_main.unwrap()
    }
    // ad hoc loc
    pub fn print_short<'eval>(&self, value: &__Register<'eval>, ty: EntityRoutePtr) -> String {
        let intrinsic_ty = ty.intrinsic();
        match intrinsic_ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::Void => todo!(),
                RootIdentifier::I32 => match value.data_kind() {
                    __RegisterDataKind::TempRef => todo!(),
                    __RegisterDataKind::TempMut => todo!(),
                    __RegisterDataKind::Moved => todo!(),
                    __RegisterDataKind::Undefined => todo!(),
                    __RegisterDataKind::Unreturned => "unreturned".to_string(),
                    _ => format!("{}", value.downcast_i32()),
                },
                RootIdentifier::I64 => todo!(),
                RootIdentifier::F32 => todo!(),
                RootIdentifier::F64 => todo!(),
                RootIdentifier::B32 => todo!(),
                RootIdentifier::B64 => todo!(),
                RootIdentifier::Bool => format!("{}", value.downcast_bool()),
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::Fp => todo!(),
                RootIdentifier::Fn => todo!(),
                RootIdentifier::FnMut => todo!(),
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::Domains => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::VisualType => todo!(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::TraitType => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::Option => todo!(),
            },
            EntityRoutePtr::Custom(_) => {
                let ty_decl: Arc<TyDecl> = self.ty_decl(intrinsic_ty).unwrap();
                match ty_decl.ty_kind {
                    TyKind::Enum => todo!(),
                    TyKind::Record => todo!(),
                    TyKind::Struct => "{ ... }".to_string(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => "[ ... ]".to_string(),
                    TyKind::Array => todo!(),
                    TyKind::Slice => todo!(),
                    TyKind::CyclicSlice => todo!(),
                    TyKind::Tuple => todo!(),
                    TyKind::Mor => todo!(),
                    TyKind::Fp => todo!(),
                    TyKind::AssociatedAny => todo!(),
                    TyKind::ThisAny => todo!(),
                    TyKind::SpatialPlaceholderAny => todo!(),
                    TyKind::BoxAny => todo!(),
                    TyKind::HigherKind => todo!(),
                    TyKind::Ref => todo!(),
                    TyKind::Option => todo!(),
                }
            }
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    // ad hoc
    pub fn vtable<'eval>(&self, ty: EntityRoutePtr) -> &'eval __RegisterTyVTable {
        unsafe {
            // ad hoc: how to do with Option<>
            match ty.intrinsic() {
                EntityRoutePtr::Root(root_identifier) => match root_identifier {
                    RootIdentifier::Void => todo!(),
                    RootIdentifier::I32 => &__I32_VTABLE,
                    RootIdentifier::I64 => todo!(),
                    RootIdentifier::F32 => todo!(),
                    RootIdentifier::F64 => todo!(),
                    RootIdentifier::B32 => todo!(),
                    RootIdentifier::B64 => todo!(),
                    RootIdentifier::Bool => todo!(),
                    RootIdentifier::True => todo!(),
                    RootIdentifier::False => todo!(),
                    RootIdentifier::Vec => todo!(),
                    RootIdentifier::Tuple => todo!(),
                    RootIdentifier::Debug => todo!(),
                    RootIdentifier::Std => todo!(),
                    RootIdentifier::Core => todo!(),
                    RootIdentifier::Mor => todo!(),
                    RootIdentifier::Fp => todo!(),
                    RootIdentifier::Fn => todo!(),
                    RootIdentifier::FnMut => todo!(),
                    RootIdentifier::FnOnce => todo!(),
                    RootIdentifier::Array => todo!(),
                    RootIdentifier::Domains => todo!(),
                    RootIdentifier::DatasetType => todo!(),
                    RootIdentifier::VisualType => todo!(),
                    RootIdentifier::TypeType => todo!(),
                    RootIdentifier::TraitType => todo!(),
                    RootIdentifier::ModuleType => todo!(),
                    RootIdentifier::CloneTrait => todo!(),
                    RootIdentifier::CopyTrait => todo!(),
                    RootIdentifier::PartialEqTrait => todo!(),
                    RootIdentifier::EqTrait => todo!(),
                    RootIdentifier::Ref => todo!(),
                    RootIdentifier::Option => todo!(),
                },
                EntityRoutePtr::Custom(_) => {
                    todo!()
                }
                EntityRoutePtr::ThisType => todo!(),
            }
        }
    }
}

pub trait AskCompileTime {
    fn compile_time(&self) -> &HuskyComptime;
}
