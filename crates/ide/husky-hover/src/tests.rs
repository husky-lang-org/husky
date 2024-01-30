pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;

use husky_entity_tree::EntityTreeJar;
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EtherealTermJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_sema_expr::SemaExprJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_defn::SynDefnJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::{TokenDb, TokenIdx, TokenJar};
use husky_token_info::TokenInfoJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;

#[salsa::db(
    VfsJar,
    CowordJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    TokenInfoJar,
    husky_entity_path::jar::EntityPathJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    SynDeclJar,
    SynDefnJar,
    SynExprJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    husky_declarative_ty::db::DeclarativeTypeJar,
    EtherealTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    SemaExprJar,
    HoverJar
)]
#[derive(Default)]
pub(crate) struct DB;

#[test]
fn hover_result_works() {
    const N: usize = 20;
    DB::ast_expect_test_debug(
        |db, module_path| -> Vec<(TokenIdx, Option<HoverResult>)> {
            let ranged_token_sheet = db.ranged_token_sheet(module_path);
            let len = ranged_token_sheet.len();
            let step = (len / N).max(1);
            let mut hover_results = vec![];
            for token_idx in ranged_token_sheet.token_index_iter() {
                // only push some of them, but all of them have to be computed
                let hover_result = calc_hover_result(db, module_path, token_idx);
                if token_idx.index() % step == 0 {
                    hover_results.push((token_idx, hover_result))
                }
            }
            hover_results
        },
        &AstTestConfig::new(
            "hover_result",
            FileExtensionConfig::Markdown,
            VfsTestDomainsConfig::Full,
        ),
    )
}
