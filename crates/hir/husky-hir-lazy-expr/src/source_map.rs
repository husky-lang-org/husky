use crate::{
    db::HirLazyExprJar, variable::HirLazyVariableIdx, HirLazyExprIdx, HirLazyPatternExprIdx,
    HirLazyStmtIdx,
};
use husky_sema_expr::{SemaExprIdx, SemaExprMap, SemaStmtIdx, SemaStmtMap};
use husky_syn_expr::{
    CurrentSynSymbolIdx, PatternSynExprIdx, SynPatternExprMap, SynPatternExprRoot, SynSymbolMap,
};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar, constructor = new_inner)]
pub struct HirLazyExprSourceMap {
    #[return_ref]
    pub data: HirLazyExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprSourceMapData {
    syn_to_hir_lazy_pattern_expr_idx_map: SynPatternExprMap<HirLazyPatternExprIdx>,
    sema_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
    sema_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
    syn_symbol_to_hir_lazy_variable_map: SynSymbolMap<HirLazyVariableIdx>,
}

impl HirLazyExprSourceMapData {
    pub fn syn_pattern_root_to_sema_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirLazyPatternExprIdx {
        self.syn_to_hir_lazy_pattern_expr_idx_map[syn_pattern_root.into().syn_pattern_expr_idx()]
    }

    pub fn syn_to_hir_lazy_pattern_expr_idx(
        &self,
        syn_pattern_expr_idx: PatternSynExprIdx,
    ) -> Option<HirLazyPatternExprIdx> {
        self.syn_to_hir_lazy_pattern_expr_idx_map
            .get(syn_pattern_expr_idx)
            .copied()
    }

    pub fn sema_to_hir_lazy_expr_idx(&self, sema_expr_idx: SemaExprIdx) -> Option<HirLazyExprIdx> {
        self.sema_to_hir_lazy_expr_idx_map
            .get(sema_expr_idx)
            .copied()
    }

    pub fn sema_to_hir_lazy_stmt_idx(&self, sema_stmt_idx: SemaStmtIdx) -> Option<HirLazyStmtIdx> {
        self.sema_to_hir_lazy_stmt_idx_map
            .get(sema_stmt_idx)
            .copied()
    }

    pub fn current_syn_symbol_to_hir_lazy_variable(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<HirLazyVariableIdx> {
        self.syn_symbol_to_hir_lazy_variable_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }

    pub fn sema_expr_idx(&self, expr: HirLazyExprIdx) -> SemaExprIdx {
        self.sema_to_hir_lazy_expr_idx_map
            .iter()
            .find_map(|(sema_expr, &expr1)| (expr == expr1).then_some(sema_expr))
            .unwrap()
    }
}

impl HirLazyExprSourceMap {
    pub fn new(
        db: &::salsa::Db,
        syn_to_hir_lazy_pattern_expr_idx_map: SynPatternExprMap<HirLazyPatternExprIdx>,
        sema_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
        sema_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
        syn_symbol_to_hir_lazy_variable_map: SynSymbolMap<HirLazyVariableIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirLazyExprSourceMapData {
                syn_to_hir_lazy_pattern_expr_idx_map,
                sema_to_hir_lazy_expr_idx_map,
                sema_to_hir_lazy_stmt_idx_map,
                syn_symbol_to_hir_lazy_variable_map,
            },
        )
    }
}
