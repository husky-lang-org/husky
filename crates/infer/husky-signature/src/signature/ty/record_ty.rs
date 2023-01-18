use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct RecordTypeSignature {
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
}

impl RecordTypeSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterSignature] { &l })
            .unwrap_or(&[])
    }
}
