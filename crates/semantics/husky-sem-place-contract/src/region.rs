#[cfg(test)]
use crate::*;
use crate::{engine::PlaceContractEngine, site::SemaPlaceContractSite};
#[cfg(test)]
use husky_sem_expr::SemExprDb;
use husky_sem_expr::{SemExprIdx, SemExprMap, SemExprRegion};
#[cfg(test)]
use husky_syn_defn::module_item_syn_defns;
#[cfg(test)]
use husky_vfs::ModulePath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaPlaceContractRegion {
    expr_sites: SemExprMap<SemaPlaceContractSite>,
}

/// # constructor
impl SemaPlaceContractRegion {
    pub(crate) fn new(expr_sites: SemExprMap<SemaPlaceContractSite>) -> Self {
        Self { expr_sites }
    }
}

impl std::ops::Index<SemExprIdx> for SemaPlaceContractRegion {
    type Output = SemaPlaceContractSite;

    fn index(&self, expr: SemExprIdx) -> &Self::Output {
        &self.expr_sites[expr]
    }
}

#[salsa::tracked(return_ref)]
pub fn sem_place_contract_region(
    db: &::salsa::Db,
    sem_expr_region: SemExprRegion,
) -> SemaPlaceContractRegion {
    let mut engine = PlaceContractEngine::new(db, sem_expr_region);
    engine.infer_all_exprs();
    engine.finish()
}

#[cfg(test)]
fn decl_sem_place_contract_regions(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<&SemaPlaceContractRegion> {
    use husky_syn_decl::HasSynDeclSheet;

    module_path
        .syn_decl_sheet(db)
        .decls(db)
        .iter()
        .copied()
        .filter_map(|(_, decl)| {
            Some(sem_place_contract_region(
                db,
                db.sem_expr_region(decl.syn_expr_region(db)?),
            ))
        })
        .collect()
}

#[test]
fn decl_sem_place_contract_regions_works() {
    DB::ast_expect_test_debug_with_db(
        decl_sem_place_contract_regions,
        &AstTestConfig::new(
            "decl_sem_place_contract_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}

#[cfg(test)]
fn defn_sem_place_contract_regions(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<&SemaPlaceContractRegion> {
    module_item_syn_defns(db, module_path)
        .iter()
        .copied()
        .filter_map(|(_, defn)| {
            Some(sem_place_contract_region(
                db,
                db.sem_expr_region(defn?.syn_expr_region),
            ))
        })
        .collect()
}

#[test]
fn defn_sem_place_contract_regions_works() {
    DB::ast_expect_test_debug_with_db(
        defn_sem_place_contract_regions,
        &AstTestConfig::new(
            "defn_sem_place_contract_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}