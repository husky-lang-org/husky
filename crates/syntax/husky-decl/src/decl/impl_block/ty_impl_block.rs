use super::*;
use husky_token::EolToken;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeImplBlockNodeDecl {
    #[id]
    pub node_path: TypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_block: TypeImplBlockNode,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub ty_expr: TypeExpr,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolToken>,
    pub expr_region: ExprRegion,
}

impl HasNodeDecl for TypeImplBlockNodePath {
    type NodeDecl = TypeImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        ty_impl_block_node_decl(db, self)
    }
}

impl HasNodeDecl for TypeImplBlockNode {
    type NodeDecl = TypeImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        self.node_path(db).node_decl(db)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ty_impl_block_node_decl(
    db: &dyn DeclDb,
    node_path: TypeImplBlockNodePath,
) -> TypeImplBlockNodeDecl {
    let parser = DeclParseContext::new(db, node_path.module_path(db));
    parser.parse_ty_impl_block_node_decl(node_path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_ty_impl_block_node_decl(
        &self,
        node_path: TypeImplBlockNodePath,
    ) -> TypeImplBlockNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self.parse_ty_impl_block_decl_aux(node_path, node, ast_idx, token_group_idx),
            _ => unreachable!(),
        }
    }

    fn parse_ty_impl_block_decl_aux(
        &self,
        node_path: TypeImplBlockNodePath,
        node: TypeImplBlockNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> TypeImplBlockNodeDecl {
        let db = self.db();
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, None);
        let impl_token = ctx.try_parse_optional().unwrap().unwrap();
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        let ty = ctx.try_parse_optional().unwrap().unwrap();
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon);
        TypeImplBlockNodeDecl::new(
            db,
            node_path,
            ast_idx,
            node,
            impl_token,
            implicit_parameter_decl_list,
            ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct TypeImplBlockDecl {
    #[id]
    pub node_path: TypeImplBlockNodePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub ty_expr: TypeExpr,
    pub expr_region: ExprRegion,
}

impl From<TypeImplBlockDecl> for Decl {
    fn from(decl: TypeImplBlockDecl) -> Self {
        Decl::ImplBlock(decl.into())
    }
}

impl HasDecl for TypeImplBlockPath {
    type Decl = TypeImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        ty_impl_block_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn ty_impl_block_decl(
    db: &dyn DeclDb,
    // here use path instead of node_path because salsa doesn't support use wrapper type by default
    // maybe add AsId carefully
    path: TypeImplBlockPath,
) -> DeclResult<TypeImplBlockDecl> {
    let node_path = path.node_path(db);
    let node_decl = node_path.node_decl(db);
    TypeImplBlockDecl::from_node_decl(db, node_path, node_decl)
}

impl TypeImplBlockDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        node_path: TypeImplBlockNodePath,
        node_decl: TypeImplBlockNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let ty_expr = node_decl.ty_expr(db);
        let expr_region = node_decl.expr_region(db);
        node_decl.eol_colon(db).as_ref()?;
        Ok(Self::new(
            db,
            node_path,
            implicit_parameters,
            ty_expr,
            expr_region,
        ))
    }
}
