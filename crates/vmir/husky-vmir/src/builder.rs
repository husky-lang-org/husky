use crate::{
    destroyer::{VmirDestroyerArena, VmirDestroyerData, VmirDestroyerIdxRange},
    expr::{VmirExprArena, VmirExprData, VmirExprIdx, VmirExprIdxRange},
    stmt::{VmirStmtArena, VmirStmtData, VmirStmtIdxRange},
};
use husky_hir_eager_expr::{HirEagerExprArena, HirEagerExprIdx, HirEagerStmtArena};
use husky_hir_expr::{HirExprIdx, HirExprRegion};
use husky_linkage::{instantiation::LinInstantiation, linkage::Linkage};

pub(crate) struct VmirExprBuilder<'db> {
    db: &'db ::salsa::Db,
    hir_eager_expr_arena: &'db HirEagerExprArena,
    hir_eager_stmt_arena: &'db HirEagerStmtArena,
    instantiation: &'db LinInstantiation,
    vmir_expr_arena: VmirExprArena,
    vmir_stmt_arena: VmirStmtArena,
    vmir_destroyer_arena: VmirDestroyerArena,
}

/// # constructor
impl<'db> VmirExprBuilder<'db> {
    pub(crate) fn new(linkage: Linkage, db: &'db ::salsa::Db) -> Option<(HirEagerExprIdx, Self)> {
        use husky_hir_defn::defn::HasHirDefn;

        let (path, instantiation) = linkage.path_and_instantiation_for_definition(db)?;
        let hir_defn = path.hir_defn(db).unwrap();
        let (HirExprIdx::Eager(body), HirExprRegion::Eager(hir_eager_expr_region)) =
            hir_defn.hir_expr_body_and_region(db)?
        else {
            return None;
        };
        Some((
            body,
            Self {
                db,
                hir_eager_expr_arena: hir_eager_expr_region.expr_arena(db),
                hir_eager_stmt_arena: hir_eager_expr_region.stmt_arena(db),
                instantiation,
                vmir_expr_arena: Default::default(),
                vmir_stmt_arena: Default::default(),
                vmir_destroyer_arena: Default::default(),
            },
        ))
    }
}

/// # getters
impl<'db> VmirExprBuilder<'db> {
    pub(crate) fn db(&self) -> &'db ::salsa::Db {
        self.db
    }

    pub(crate) fn hir_eager_expr_arena(&self) -> &'db HirEagerExprArena {
        self.hir_eager_expr_arena
    }

    pub(crate) fn hir_eager_stmt_arena(&self) -> &'db HirEagerStmtArena {
        self.hir_eager_stmt_arena
    }

    pub(crate) fn instantiation(&self) -> &'db LinInstantiation {
        self.instantiation
    }
}

/// # actions
impl<'db> VmirExprBuilder<'db> {
    pub(crate) fn alloc_expr(&mut self, expr_data: VmirExprData) -> VmirExprIdx {
        self.vmir_expr_arena.alloc_one(expr_data)
    }

    pub(crate) fn alloc_exprs(&mut self, expr_datas: Vec<VmirExprData>) -> VmirExprIdxRange {
        self.vmir_expr_arena.alloc_batch(expr_datas)
    }

    pub(crate) fn alloc_stmts(&mut self, stmts: Vec<VmirStmtData>) -> VmirStmtIdxRange {
        self.vmir_stmt_arena.alloc_batch(stmts)
    }

    pub(crate) fn alloc_destroyers(
        &mut self,
        destroyer_datas: Vec<VmirDestroyerData>,
    ) -> VmirDestroyerIdxRange {
        self.vmir_destroyer_arena.alloc_batch(destroyer_datas)
    }

    pub(crate) fn finish(self) -> (VmirExprArena, VmirStmtArena) {
        (self.vmir_expr_arena, self.vmir_stmt_arena)
    }
}
