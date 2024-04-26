use super::*;
use husky_eth_term::term::ritchie::EthRitchieSimpleParameter;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodRitchieEthTemplate {
    pub path: TypeItemPath,
    pub self_ty: EthTerm,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_value_parameter: EthRitchieSimpleParameter,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}

impl TypeMethodRitchieEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeItemPath,
        tmpl: TypeMethodRitchieDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EthTerm::ty_from_dec(db, tmpl.self_ty(db))?;
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        let self_value_parameter =
            EthRitchieSimpleParameter::from_dec(db, tmpl.self_value_parameter(db))?;
        let parenate_parameters =
            EtherealParenateParameters::from_dec(db, tmpl.parenate_parameters(db))?;
        let return_ty = EthTerm::ty_from_dec(db, tmpl.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}