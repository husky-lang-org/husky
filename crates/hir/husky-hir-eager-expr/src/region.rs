use husky_entity_path::region::RegionPath;

use crate::{
    var::{cvar::HirEagerComptimeSvarRegionData, rvar::HirEagerRuntimeSvarRegionData},
    *,
};

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {
    pub region_path: RegionPath,
    #[return_ref]
    pub expr_arena: HirEagerExprArena,
    #[return_ref]
    pub stmt_arena: HirEagerStmtArena,
    #[return_ref]
    pub pattern_expr_arena: HirEagerPatternExprArena,
    #[return_ref]
    pub comptime_symbol_region_data: HirEagerComptimeSvarRegionData,
    #[return_ref]
    pub runtime_symbol_region_data: HirEagerRuntimeSvarRegionData,
}
