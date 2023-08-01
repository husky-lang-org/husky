use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    pub associated_ty: EtherealTerm,
}

impl TraitForTypeAssociatedTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TraitForTypeItemPath,
        declarative_signature_template: TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        let ty_term =
            EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty_term(db))?;
        Ok(Self::new(db, path, template_parameters, ty_term))
    }

    pub(super) fn inherit_partial_instantiation(
        self,
        db: &dyn EtherealSignatureDb,
        impl_block_template_partially_instantiated: TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    ) -> TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated {
        let partial_instantiation = impl_block_template_partially_instantiated
            .partial_instantiation(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated::new(
            db,
            self,
            partial_instantiation,
        )
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = pub(super) new)]
pub struct TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated {
    pub template: TraitForTypeAssociatedTypeEtherealSignatureTemplate,
    #[return_ref]
    pub partial_instantiation: EtherealTermPartialInstantiation,
}

impl TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated {
    pub fn try_into_signature(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
        trai_for_ty_associated_ty_ethereal_signature_template_partially_instantiated_try_into_signature(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn trai_for_ty_associated_ty_ethereal_signature_template_partially_instantiated_try_into_signature(
    db: &dyn EtherealSignatureDb,
    template_partially_instantiated: TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated,
) -> Option<TraitForTypeAssociatedTypeEtherealSignature> {
    let instantiation = template_partially_instantiated
        .partial_instantiation(db)
        .try_into_instantiation()?;
    let template = template_partially_instantiated.template(db);
    Some(TraitForTypeAssociatedTypeEtherealSignature {
        ty_term: template.associated_ty(db).instantiate(db, &instantiation),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeAssociatedTypeEtherealSignature {
    ty_term: EtherealTerm,
}

impl TraitForTypeAssociatedTypeEtherealSignature {
    pub fn ty_term(&self) -> EtherealTerm {
        self.ty_term
    }
}
