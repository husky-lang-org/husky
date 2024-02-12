use super::*;
use husky_hir_lazy_expr::HirLazyExprRegion;
use husky_syn_decl::decl::MajorGnSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct FunctionGnFugitiveHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirLazyParenateParameters,
    pub return_ty: HirType,
    pub hir_lazy_expr_region: HirLazyExprRegion,
}

impl FunctionGnFugitiveHirDecl {
    pub(super) fn from_syn(path: FugitivePath, syn_decl: MajorGnSynDecl, db: &::salsa::Db) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let parenate_parameters =
            HirLazyParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = builder.return_ty_before_colon(syn_decl.return_ty(db));
        Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
            builder.finish().lazy(),
        )
    }
}
