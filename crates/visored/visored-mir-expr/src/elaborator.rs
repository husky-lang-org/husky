pub mod linear;

use crate::{
    expr::VdMirExprIdx,
    hypothesis::constructor::VdMirHypothesisConstructor,
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtIdx, VdMirStmtIdxRange},
    *,
};

pub trait IsVdMirTacticElaborator<'db> {
    type HypothesisIdx;

    fn elaborate_stmts_ext(
        self,
        stmts: VdMirStmtIdxRange,
        hypothesis_constructor: &mut VdMirHypothesisConstructor<'db, Self::HypothesisIdx>,
    );
    fn elaborate_stmt_ext(
        self,
        stmt: VdMirStmtIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor<'db, Self::HypothesisIdx>,
    );
    fn elaborate_expr_ext(
        self,
        expr: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor<'db, Self::HypothesisIdx>,
    );
}

pub type VdMirTrivialElaborator<'db> = self::linear::VdMirSequentialElaborator<'db, ()>;
