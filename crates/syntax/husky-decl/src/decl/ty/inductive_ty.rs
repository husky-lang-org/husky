use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct InductiveTypeDecl {
    pub path: TypePath,
}
