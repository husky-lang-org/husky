use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct SubmoduleNodeDecl {
    #[id]
    pub node_path: SubmoduleNodePath,
    pub ast_idx: AstIdx,
}

impl HasNodeDecl for SubmoduleNodePath {
    type NodeDecl = SubmoduleNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        submodule_node_decl(db, self)
    }
}

#[salsa::tracked( jar = DeclJar)]
pub(crate) fn submodule_node_decl(
    db: &dyn DeclDb,
    node_path: SubmoduleNodePath,
) -> SubmoduleNodeDecl {
    let node = node_path.node(db);
    SubmoduleNodeDecl::new(db, node_path, node.ast_idx(db))
}

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct SubmoduleDecl {
    #[id]
    pub path: ModulePath,
    pub ast_idx: AstIdx,
}

impl SubmoduleDecl {
    #[inline(always)]
    fn from_node_decl(db: &dyn DeclDb, path: ModulePath, node_decl: SubmoduleNodeDecl) -> Self {
        Self::new(db, path, node_decl.ast_idx(db))
    }
}

// actually it only works for nonroot module path
// but rust doesn't provide refinement type
impl HasDecl for ModulePath {
    type Decl = SubmoduleDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        submodule_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn submodule_decl(db: &dyn DeclDb, path: ModulePath) -> DeclResult<SubmoduleDecl> {
    let node_path = path.node_path(db);
    let node_decl = node_path.node_decl(db);
    Ok(SubmoduleDecl::from_node_decl(db, path, node_decl))
}
