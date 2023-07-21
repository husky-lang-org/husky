use original_error::IntoError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct IllFormedImplBlockSynNodePath {
    path: IllFormedImplBlockPath,
}

impl salsa::AsId for IllFormedImplBlockSynNodePath {
    fn as_id(self) -> salsa::Id {
        self.path.as_id()
    }

    fn from_id(id: salsa::Id) -> Self {
        IllFormedImplBlockSynNodePath {
            path: IllFormedImplBlockPath::from_id(id),
        }
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for IllFormedImplBlockSynNodePath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl IllFormedImplBlockSynNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn item_node_paths(self, db: &dyn EntityTreeDb) -> &[IllFormedItemSynNodePath] {
        // ad hoc
        &[]
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> IllFormedImplBlockSynNode {
        ill_formed_impl_block_node(db, self)
    }
}

impl From<IllFormedImplBlockSynNodePath> for EntitySynNodePath {
    fn from(id: IllFormedImplBlockSynNodePath) -> Self {
        EntitySynNodePath::ImplBlock(id.into())
    }
}

impl HasSynNodePath for IllFormedImplBlockPath {
    type SynNodePath = IllFormedImplBlockSynNodePath;

    fn syn_node_path(self, db: &dyn EntityTreeDb) -> Self::SynNodePath {
        IllFormedImplBlockSynNodePath { path: self }
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedImplBlockSynNode {
    #[id]
    pub node_path: IllFormedImplBlockSynNodePath,
    pub impl_token: ImplToken,
    pub ast_idx: AstIdx,
    pub items: Option<ImplBlockItems>,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlockSynNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        impl_token: ImplToken,
        module: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        IllFormedImplBlockSynNode::new_inner(
            db,
            IllFormedImplBlockSynNodePath {
                path: IllFormedImplBlockPath::new(db, registry, module),
            },
            impl_token,
            ast_idx,
            items,
            ill_form,
        )
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplBlockIllForm {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
    #[error("MissingFor")]
    MissingForKeyword,
    #[error("ExpectTypePathAfterFor")]
    ExpectTypePathAfterForKeyword,
    #[error("expected `derive` identifier")]
    ExpectedDeriveIdent(TokenStreamState),
}

impl IntoError for ImplBlockIllForm {
    type Error = Self;
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ill_formed_impl_block_node(
    db: &dyn EntityTreeDb,
    node_path: IllFormedImplBlockSynNodePath,
) -> IllFormedImplBlockSynNode {
    let module_path = node_path.module_path(db);
    let entity_tree_sheet = db.entity_tree_sheet(module_path).expect("valid module");
    entity_tree_sheet.ill_formed_impl_block_node(db, node_path)
}
