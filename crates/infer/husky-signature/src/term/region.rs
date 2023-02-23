use super::*;
use husky_entity_tree::RegionPath;
use husky_expr::{CurrentSymbolIdx, ExprIdx, ExprMap, ExprRegion};

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermRegion {
    path: RegionPath,
    term_symbol_region: TermSymbolRegion,
    expr_terms: ExprMap<SignatureTermResult<Term>>,
}

impl SignatureTermRegion {
    pub fn new(
        path: RegionPath,
        term_symbol_region: TermSymbolRegion,
        expr_terms: ExprMap<SignatureTermResult<Term>>,
    ) -> Self {
        Self {
            path,
            term_symbol_region,
            expr_terms,
        }
    }

    pub fn term_symbol_region(&self) -> &TermSymbolRegion {
        &self.term_symbol_region
    }

    pub fn current_symbol_term(&self, current_symbol_idx: CurrentSymbolIdx) -> Option<TermSymbol> {
        self.term_symbol_region
            .current_symbol_term(current_symbol_idx)
    }

    pub fn expr_term(&self, expr: ExprIdx) -> SignatureTermResultBorrowed<Term> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }
}

#[salsa::tracked(jar = SignatureJar, return_ref)]
pub(crate) fn signature_term_region(
    db: &dyn SignatureDb,
    expr_region: ExprRegion,
) -> SignatureTermRegion {
    let expr_region_data = expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| signature_term_region(db, r).term_symbol_region());
    SignatureTermEngine::new(db, expr_region, parent_term_symbol_region).finish()
}
