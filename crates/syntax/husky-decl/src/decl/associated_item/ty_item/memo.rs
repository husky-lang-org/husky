use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMemoDecl {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolColonToken,
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeMemoDecl> {
        let Ok(impl_decl) = associated_item.impl_block(self.db()).decl(
            self.db()
        ) else { todo!() };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };

        let curry_token = ctx.parse()?;
        let return_ty = if curry_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectOutputType)?)
        } else {
            None
        };
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon)?;
        Ok(TypeMemoDecl::new(
            self.db(),
            path,
            associated_item,
            ast_idx,
            parser.finish(),
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }
}
