use husky_print_utils::p;
use vec_like::VecPairMap;

use crate::*;

#[salsa::tracked(db = SemaExprDb, jar = SemaExprJar, constructor = new_inner)]
pub struct SemaExprRegion {
    #[id]
    pub path: RegionPath,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    pub data: SemaExprRegionData,
}

impl SemaExprRegion {
    pub(crate) fn new(
        path: RegionPath,
        syn_expr_region: SynExprRegion,
        sema_expr_arena: SemaExprArena,
        sema_stmt_arena: SemaStmtArena,
        sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, ExprRootKind)>,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
        symbol_tys: SymbolMap<SymbolType>,
        symbol_terms: SymbolMap<FluffyTerm>,
        fluffy_term_region: FluffyTermRegion,
        return_ty: Option<EtherealTerm>,
        self_ty: Option<EtherealTerm>,
        db: &dyn SemaExprDb,
    ) -> Self {
        SemaExprRegion::new_inner(
            db,
            path,
            syn_expr_region,
            SemaExprRegionData {
                path,
                sema_expr_arena,
                sema_stmt_arena,
                sema_expr_roots,
                pattern_expr_ty_infos,
                pattern_symbol_ty_infos,
                sema_expr_terms,
                symbol_tys,
                symbol_terms,
                fluffy_term_region,
                return_ty,
                self_ty,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprRegionData {
    path: RegionPath,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, ExprRootKind)>,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FluffyTerm>,
    fluffy_term_region: FluffyTermRegion,
    return_ty: Option<EtherealTerm>,
    self_ty: Option<EtherealTerm>,
}

impl SemaExprRegionData {
    pub fn syn_root_to_sema_expr_idx(&self, syn_expr_root: SynExprIdx) -> SemaExprIdx {
        self.sema_expr_roots[syn_expr_root].1 .0
    }

    pub fn sema_expr_roots<'a>(&'a self) -> impl Iterator<Item = (SemaExprIdx, ExprRootKind)> + 'a {
        self.sema_expr_roots.iter().map(|&(_, root)| root)
    }

    pub fn sema_expr_arena(&self) -> SemaExprArenaRef {
        self.sema_expr_arena.arena_ref()
    }

    pub fn sema_stmt_arena(&self) -> SemaStmtArenaRef {
        self.sema_stmt_arena.arena_ref()
    }

    pub fn sema_expr_term(
        &self,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<SemaExprTermResultRef<FluffyTerm>> {
        Some(
            self.sema_expr_terms
                .get_value(sema_expr_idx)?
                .as_ref()
                .copied(),
        )
    }

    pub fn fluffy_term_region(&self) -> &FluffyTermRegion {
        &self.fluffy_term_region
    }

    pub fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub fn symbol_terms(&self) -> &SymbolMap<FluffyTerm> {
        &self.symbol_terms
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn sema_expr_terms(&self) -> &VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>> {
        &self.sema_expr_terms
    }
}

#[salsa::tracked(jar = SemaExprJar)]
pub(crate) fn sema_expr_region(
    db: &dyn SemaExprDb,
    syn_expr_region: SynExprRegion,
) -> SemaExprRegion {
    let mut engine = SemaExprEngine::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct PatternExprTypeInfo {
    ty: PatternSemaExprResult<FluffyTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternSemaExprResult<FluffyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FluffyTerm, &PatternSemaExprError> {
        self.ty.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FluffyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }
}
