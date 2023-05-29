use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterEtherealSignatures,
    #[return_ref]
    pub self_parameter: ExplicitParameterEtherealSignatureTemplate,
    #[return_ref]
    pub nonself_regular_parameters: ExplicitParameterEtherealSignatureTemplates,
    pub return_ty: EtherealTerm,
}

impl HasEtherealSignatureTemplate for TypeMethodFnDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = TypeMethodFnEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_method_fn_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_method_fn_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature: TypeMethodFnDeclarativeSignatureTemplate,
) -> EtherealSignatureResult<TypeMethodFnEtherealSignatureTemplate> {
    let self_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
    let implicit_parameters = ImplicitParameterEtherealSignatures::from_declarative(
        db,
        declarative_signature.implicit_parameters(db),
    )?;
    let self_parameter = ExplicitParameterEtherealSignatureTemplate::from_declarative(
        db,
        declarative_signature.self_parameter(db),
    )?;
    let nonself_regular_parameters = ExplicitParameterEtherealSignatureTemplates::from_declarative(
        db,
        declarative_signature.nonself_regular_parameters(db),
    )?;
    let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
    Ok(TypeMethodFnEtherealSignatureTemplate::new(
        db,
        self_ty,
        implicit_parameters,
        self_parameter,
        nonself_regular_parameters,
        return_ty,
    ))
}
