use crate::*;
use husky_hir_ty::db::HirTypeDb;
use husky_sema_expr::SemaExprDb;

pub trait HirEagerExprDb: salsa::DbWithJar<HirEagerExprJar> + SemaExprDb + HirTypeDb {}

impl HirEagerExprDb for Db where Db: salsa::DbWithJar<HirEagerExprJar> + SemaExprDb + HirTypeDb {}

#[salsa::jar(db = HirEagerExprDb)]
pub struct HirEagerExprJar(
    HirEagerExprRegion,
    HirEagerExprSourceMap,
    hir_eager_expr_region_with_source_map,
);
