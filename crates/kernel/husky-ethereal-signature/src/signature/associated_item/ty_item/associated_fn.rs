use super::*;
use husky_term_prelude::RitchieKind;

#[salsa::tracked(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedFnEtherealSignatureTemplate {
    #[id]
    pub path: TypeItemPath,
    pub self_ty: EtherealTerm,
    pub generic_parameters: EtherealGenericParameters,
    pub explicit_parameters: SpecificParameterEtherealSignatureTemplates,
    pub return_ty: EtherealTerm,
    pub ty: EtherealTerm,
}

impl TypeAssociatedFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeItemPath,
        declarative_signature: TypeAssociatedFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
        let generic_parameters = EtherealGenericParameters::from_declarative(
            db,
            declarative_signature.generic_parameters(db),
        )?;
        let explicit_parameters = SpecificParameterEtherealSignatureTemplates::from_declarative(
            db,
            declarative_signature.explicit_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        let ty = EtherealTermRitchie::new(
            db,
            RitchieKind::FnType,
            explicit_parameters
                .iter()
                .copied()
                .map(|parameter| match parameter {
                    SpecificEtherealParameter::Regular(parameter) => {
                        EtherealTermRitchieRegularParameter::new(
                            parameter.contract(),
                            parameter.ty(),
                        )
                        .into()
                    }
                    SpecificEtherealParameter::Variadic(_) => todo!(),
                    SpecificEtherealParameter::Keyed(_) => todo!(),
                }),
            return_ty,
        )?
        .into();
        Ok(Self::new(
            db,
            path,
            self_ty,
            generic_parameters,
            explicit_parameters,
            return_ty,
            ty,
        ))
    }
}
