#[salsa::db(
    husky_coword::jar::CowordJar,
    visored_zfs_ty::jar::VdZfsTypeJar,
    visored_opr::jar::VdOprJar,
    latex_ast::jar::LxAstJar,
    crate::jar::VdSemExprJar,
    visored_syn_expr::jar::VdSynExprJar
)]
pub(crate) struct DB {}
