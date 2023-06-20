use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeImplBlockNodeDecl {
    #[id]
    pub node_path: TraitForTypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub trai_expr: TraitExpr,
    pub for_token: ConnectionForToken,
    pub ty_expr: TypeExpr,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolToken>,
    pub expr_region: ExprRegion,
}

impl HasNodeDecl for TraitForTypeImplBlockNodePath {
    type NodeDecl = TraitForTypeImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        trai_for_ty_impl_block_node_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn trai_for_ty_impl_block_node_decl(
    db: &dyn DeclDb,
    node_path: TraitForTypeImplBlockNodePath,
) -> TraitForTypeImplBlockNodeDecl {
    let parser = DeclParseContext::new(db, node_path.module_path(db));
    parser.parse_trai_for_ty_impl_block_node_decl(node_path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_trai_for_ty_impl_block_node_decl(
        &self,
        node_path: TraitForTypeImplBlockNodePath,
    ) -> TraitForTypeImplBlockNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self
                .parse_trai_for_ty_impl_block_node_decl_aux(
                    node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                )
                .into(),
            _ => unreachable!(),
        }
    }

    fn parse_trai_for_ty_impl_block_node_decl_aux(
        &self,
        node_path: TraitForTypeImplBlockNodePath,
        node: TraitForTypeImplBlockNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> TraitForTypeImplBlockNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            node.node_path(db),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, None);
        let impl_token = ctx.try_parse_optional().unwrap().unwrap();
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        // ad hoc
        let trai: TraitExpr = ctx.try_parse_optional().unwrap().unwrap();
        let for_token = ctx
            .try_parse_optional()
            .expect("guaranteed by parsing")
            .expect("guaranteed by parsing");
        let ty = ctx.try_parse_optional().unwrap().unwrap();
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon);
        TraitForTypeImplBlockNodeDecl::new(
            db,
            node_path,
            ast_idx,
            impl_token,
            implicit_parameter_decl_list,
            trai,
            for_token,
            ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct TraitForTypeImplBlockDecl {
    #[id]
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub trai_expr: TraitExpr,
    pub ty_expr: TypeExpr,
    pub expr_region: ExprRegion,
}

impl HasDecl for TraitForTypeImplBlockPath {
    type Decl = TraitForTypeImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        trai_for_ty_impl_block_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn trai_for_ty_impl_block_decl(
    db: &dyn DeclDb,
    path: TraitForTypeImplBlockPath,
) -> DeclResult<TraitForTypeImplBlockDecl> {
    let node_decl = path.node_path(db).node_decl(db);
    TraitForTypeImplBlockDecl::from_node_decl(db, path, node_decl)
}

impl TraitForTypeImplBlockDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeImplBlockPath,
        node_decl: TraitForTypeImplBlockNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let trai_expr = node_decl.trai_expr(db);
        let ty_expr = node_decl.ty_expr(db);
        let expr_region = node_decl.expr_region(db);
        node_decl.eol_colon(db).as_ref()?;
        Ok(Self::new(
            db,
            path,
            implicit_parameters,
            trai_expr,
            ty_expr,
            expr_region,
        ))
    }
}
