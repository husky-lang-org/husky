use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumPropsTypeVariantDeclarativeSignatureTemplate {
    pub parent_ty_template: EnumDeclarativeSignatureTemplate,
}
