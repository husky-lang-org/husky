use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct GnNodeDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    explicit_parameter_decl_list: NodeDeclResult<SelfParameterAndExplicitParameters<false>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExpr>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
}

impl GnNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.explicit_parameter_decl_list(db)
                        .as_ref()
                        .err()
                        .into_iter(),
                )
                .chain(self.return_ty(db).as_ref().err().into_iter())
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_gn_node_decl(
        &self,
        node_path: FugitiveNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> GnNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::False, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        let parameter_decl_list =
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedParameterDeclList);

        let curry_token = ctx.try_parse_optional();
        let return_ty = if let Ok(Some(_)) = curry_token {
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = ctx.try_parse_expected(OriginalNodeDeclError::ExpectedEolColon);
        GnNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct GnDecl {
    #[id]
    pub path: FugitivePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub regular_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl GnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: FugitivePath,
        node_decl: GnNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let explicit_parameter_decl_list = node_decl.explicit_parameter_decl_list(db).as_ref()?;
        let regular_parameters: ExplicitParameterDeclPatterns = explicit_parameter_decl_list
            .regular_parameters()
            .to_smallvec();
        let return_ty = *node_decl.return_ty(db).as_ref()?;
        let expr_region = node_decl.expr_region(db);
        Ok(GnDecl::new(
            db,
            path,
            implicit_parameters,
            regular_parameters,
            return_ty,
            expr_region,
        ))
    }
}
