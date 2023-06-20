use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct FugitiveNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
}

impl From<FugitiveNodePath> for EntityNodePath {
    fn from(id: FugitiveNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}

impl HasNodePath for FugitivePath {
    type NodePath = FugitiveNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        FugitiveNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl FugitiveNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: FugitivePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ModuleItemNode {
        fugitive_node(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn fugitive_node(db: &dyn EntityTreeDb, node_path: FugitiveNodePath) -> ModuleItemNode {
    let module_path = node_path.module_path(db);
    let entity_sheet = module_path.entity_tree_sheet(db).expect("valid file");
    match entity_sheet
        .major_entity_node(node_path.into())
        .expect("should be some")
    {
        EntityNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}
