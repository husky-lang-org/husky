use super::*;
use husky_expr::ExprIdx;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub fields: Vec<TupleStructFieldDecl>,
}

impl TupleStructTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}
