use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct StructureTypeDecl {
    pub path: TypePath,
}
