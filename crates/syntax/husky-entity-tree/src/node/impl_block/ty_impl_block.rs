use super::*;

use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntitySynTreeJar)]
#[salsa::deref_id]
pub struct TypeImplBlockSynNodePath(ItemSynNodePathId);

// basically a wrapper type
#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeImplBlockSynNodePathData {
    pub(crate) path: TypeImplBlockPath,
}

impl From<TypeImplBlockSynNodePath> for ItemSynNodePath {
    #[inline(always)]
    fn from(id: TypeImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

impl TypeImplBlockSynNodePath {
    pub fn data(self, db: &::salsa::Db) -> TypeImplBlockSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TypeImplBlock(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> TypeImplBlockPath {
        self.data(db).path
    }

    pub fn ty_path(self, db: &::salsa::Db) -> TypePath {
        self.path(db).ty_path(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a TypeImplBlockSynNode {
        let module_path = self.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        item_tree_sheet.ty_impl_block_syn_node(self)
    }

    pub(crate) fn associated_items(
        self,
        db: &::salsa::Db,
    ) -> &[(Ident, TypeItemSynNodePath, TypeItemSynNode)] {
        ty_impl_block_items(db, self)
    }

    #[inline(always)]
    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TypeItemSynNodePath> + 'a {
        self.associated_items(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl TypeImplBlockSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TypeImplBlockSynNodePath {
        TypeImplBlockSynNodePath(id)
    }

    pub fn path(self) -> TypeImplBlockPath {
        self.path
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TypeImplBlockSynNodePath(id).syn_node(db).ast_idx
    }
}

impl HasSynNodePath for TypeImplBlockPath {
    type SynNodePath = TypeImplBlockSynNodePath;

    #[inline(always)]
    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeImplBlockSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TypeImplBlock(
                TypeImplBlockSynNodePathData { path: self },
            )),
        ))
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TypeImplBlockSynNode {
    pub(crate) syn_node_path: TypeImplBlockSynNodePath,
    pub(crate) ast_idx: AstIdx,
    pub(crate) impl_token: ImplToken,
    pub(crate) ty_expr: MajorItemPathExprIdx,
    pub(crate) items: TypeItems,
}

impl TypeImplBlockSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
        impl_token: ImplToken,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: TypeItems,
        ty_path: TypePath,
        ty_expr: MajorItemPathExprIdx,
    ) -> Self {
        Self {
            syn_node_path: TypeImplBlockSynNodePath(ItemSynNodePathId::new(
                db,
                ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TypeImplBlock(
                    TypeImplBlockSynNodePathData {
                        path: TypeImplBlockPath::new(db, registry, module_path, ty_path),
                    },
                )),
            )),
            ast_idx,
            impl_token,
            ty_expr,
            items,
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.syn_node_path.path(db).module_path(db)
    }

    pub fn ty_path(self, db: &::salsa::Db) -> TypePath {
        self.syn_node_path.path(db).ty_path(db)
    }

    pub(crate) fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }
}

impl HasAssociatedItemPaths for TypeImplBlockPath {
    type AssociatedItemPath = TypeItemPath;

    fn associated_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssociatedItemPath)] {
        ty_impl_block_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_impl_block_item_paths(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> SmallVecPairMap<Ident, TypeItemPath, 2> {
    path.syn_node_path(db)
        .associated_items(db)
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}