pub(crate) use husky_ast::test_utils::AstTestConfig;
pub(crate) use husky_vfs::{test_utils::VfsTestUtils, ModulePath};

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_declarative_ty::DeclarativeTypeJar;
use husky_entity_path::{EntityPathJar, MajorItemPath, TypePath};
use husky_entity_syn_tree::{helpers::paths::module_item_paths, EntitySynTreeDb, EntitySynTreeJar};
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EtherealTermJar;
use husky_fluffy_term::FluffyTermJar;
use husky_hir_defn::HasHirDefn;
use husky_hir_defn::HirDefn;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_print_utils::p;
use husky_sema_expr::SemaExprJar;
use husky_syn_decl::{SynDeclDb, SynDeclJar};
use husky_syn_defn::SynDefnJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::VfsJar;

#[salsa::db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    AstJar,
    EntitySynTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    DeclarativeTypeJar,
    EtherealTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    SemaExprJar,
    husky_hir_ty::db::HirTypeJar,
    husky_hir_eager_expr::db::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    husky_hir_decl::db::HirDeclJar,
    husky_hir_defn::db::HirDefnJar,
    // linkage
    husky_linkage_path::db::LinkagePathJar,
    // val
    husky_val::db::ValJar,
    crate::db::ValReprJar,
)]
#[derive(Default)]
pub struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}