use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedValNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedValDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}
