use super::*;
use husky_dec_signature::signature::major_item::ty::props_struct::{
    PropsStructDecTemplate, PropsStructFieldDecTemplate,
};
use husky_eth_term::term::ritchie::EthRitchie;

#[salsa::interned]
pub struct PropsStructEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldEthTemplate; 4]>,
    pub instance_constructor_ritchie_ty: EthRitchie,
}

impl HasPropsFieldEtherealSignature for PropsStructEthTemplate {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EthTerm],
        ident: Ident,
    ) -> EthSignatureMaybeResult<PropsFieldEtherealSignature> {
        let field = self
            .fields(db)
            .iter()
            .copied()
            .find(|field| field.ident == ident)?;
        JustOk(
            field
                .instantiate(self.template_parameters(db), arguments)
                .into(),
        )
    }
}

impl PropsStructEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: PropsStructDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        let fields = tmpl
            .fields(db)
            .iter()
            .copied()
            .map(|dec_template| PropsFieldEthTemplate::from_dec(db, dec_template))
            .collect::<EthSignatureResult<_>>()?;
        let instance_constructor_ritchie_ty =
            EthRitchie::from_dec(db, tmpl.instance_constructor_ritchie_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct PropsFieldEthTemplate {
    ident: Ident,
    ty: EthTerm,
}

impl PropsFieldEthTemplate {
    fn from_dec(
        db: &::salsa::Db,
        dec_template: PropsStructFieldDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self {
            ident: dec_template.ident(),
            ty: EthTerm::ty_from_dec(db, dec_template.ty())?,
        })
    }

    // todo: move this to trait
    fn instantiate(
        self,
        template_parameters: &EthTemplateParameters,
        arguments: &[EthTerm],
    ) -> PropsStructFieldEtherealSignature {
        if template_parameters.data().len() != arguments.len() {
            todo!()
        }

        if template_parameters.data().len() == 0 {
            return PropsStructFieldEtherealSignature {
                ident: self.ident,
                ty: self.ty,
            };
        }
        todo!()
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PropsStructFieldEtherealSignature {
    ident: Ident,
    ty: EthTerm,
}

impl PropsStructFieldEtherealSignature {
    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}
