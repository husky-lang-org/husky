use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OutputTypeExpr {
    expr: ExprIdx,
}

impl OutputTypeExpr {
    pub fn expr(&self) -> ArenaIdx<Expr> {
        self.expr
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for OutputTypeExpr {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        if let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) {
            ctx.add_ty_constraint(TypeConstraint::OutputTypeExpr { expr });
            Ok(Some(OutputTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
