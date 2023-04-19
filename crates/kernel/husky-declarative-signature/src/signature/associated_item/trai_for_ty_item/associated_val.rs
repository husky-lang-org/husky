use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub(crate) fn trai_for_ty_associated_val_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeAssociatedValDecl,
) -> DeclarativeSignatureResult<TraitForTypeAssociatedValDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitForTypeAssociatedValDeclarativeSignatureTemplate::new(
        db,
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct TraitForTypeAssociatedValDeclarativeSignatureTemplate {}
