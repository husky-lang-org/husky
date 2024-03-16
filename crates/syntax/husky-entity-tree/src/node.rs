mod assoc_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

pub use self::assoc_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use enum_class::Room32;
use husky_token::IdentToken;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum ItemSynNodePath {
    Submodule(Room32, SubmoduleSynNodePath),
    MajorItem(MajorItemSynNodePath),
    TypeVariant(Room32, TypeVariantSynNodePath),
    ImplBlock(ImplBlockSynNodePath),
    AssocItem(AssocItemSynNodePath),
    Attr(Room32, AttrSynNodePath),
}

impl std::ops::Deref for ItemSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.2
    }
}

#[salsa::interned(jar = EntityTreeJar)]
pub struct ItemSynNodePathId {
    data: ItemSynNodePathData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemSynNodePathData {
    Submodule(SubmoduleSynNodePathData),
    MajorItem(MajorItemSynNodePathData),
    TypeVariant(TypeVariantSynNodePathData),
    ImplBlock(ImplBlockSynNodePathData),
    AssocItem(AssocItemSynNodePathData),
    Attr(AttrSynNodePathData),
}

impl ItemSynNodePathId {
    pub fn syn_node_path(self, db: &::salsa::Db) -> ItemSynNodePath {
        match self.data(db) {
            ItemSynNodePathData::Submodule(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::MajorItem(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::TypeVariant(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::ImplBlock(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::AssocItem(data) => data.syn_node_path(self).into(),
            ItemSynNodePathData::Attr(data) => data.syn_node_path(self).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> Option<ItemPath> {
        self.data(db).path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.data(db).module_path(db)
    }

    pub(crate) fn ast_idx(self, db: &::salsa::Db) -> AstIdx {
        self.data(db).ast_idx(self, db)
    }
}

#[test]
fn syn_node_path_id_conversion_works() {
    use crate::helpers::paths::module_item_syn_node_paths;

    DB::ast_plain_test(
        |db, module_path| {
            for &syn_node_path in module_item_syn_node_paths(db, module_path) {
                assert_eq!(syn_node_path.syn_node_path(db), syn_node_path);
            }
        },
        &AstTestConfig::new(
            "syn_node_path_id_conversion",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}

impl ItemSynNodePathData {
    pub fn path(self) -> Option<ItemPath> {
        match self {
            ItemSynNodePathData::Submodule(slf) => slf.path().map(Into::into),
            ItemSynNodePathData::MajorItem(slf) => slf.path().map(Into::into),
            ItemSynNodePathData::TypeVariant(slf) => slf.path().map(Into::into),
            ItemSynNodePathData::ImplBlock(slf) => slf.path().map(Into::into),
            ItemSynNodePathData::AssocItem(slf) => slf.path().map(Into::into),
            ItemSynNodePathData::Attr(slf) => slf.path().map(Into::into),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            ItemSynNodePathData::Submodule(slf) => slf.module_path(db),
            ItemSynNodePathData::MajorItem(slf) => slf.module_path(db),
            ItemSynNodePathData::TypeVariant(slf) => slf.module_path(db),
            ItemSynNodePathData::ImplBlock(slf) => slf.module_path(db),
            ItemSynNodePathData::AssocItem(slf) => slf.module_path(db),
            ItemSynNodePathData::Attr(slf) => slf.module_path(db),
        }
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        match self {
            ItemSynNodePathData::Submodule(slf) => slf.ast_idx(id, db),
            ItemSynNodePathData::MajorItem(slf) => slf.ast_idx(id, db),
            ItemSynNodePathData::TypeVariant(slf) => slf.ast_idx(id, db),
            ItemSynNodePathData::ImplBlock(slf) => slf.ast_idx(id, db),
            ItemSynNodePathData::AssocItem(slf) => slf.ast_idx(id, db),
            ItemSynNodePathData::Attr(slf) => slf.ast_idx(id, db),
        }
    }
}

impl ItemSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<ItemPath> {
        match self {
            ItemSynNodePath::Submodule(_, syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::TypeVariant(_, syn_node_path) => {
                syn_node_path.unambiguous_path(db).map(Into::into)
            }
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::AssocItem(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::Attr(_, syn_node_path) => syn_node_path.path(db).map(Into::into),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub(crate) fn attr_syn_nodes(self, db: &::salsa::Db) -> &[(AttrSynNodePath, AttrSynNode)] {
        // ad hoc
        match self {
            ItemSynNodePath::Submodule(_, _) => &[],
            ItemSynNodePath::MajorItem(path) => path.attrs(db),
            ItemSynNodePath::TypeVariant(_, _) => &[],
            ItemSynNodePath::ImplBlock(_) => &[],
            ItemSynNodePath::AssocItem(_) => &[],
            ItemSynNodePath::Attr(_, _) => &[],
        }
    }
}

pub trait HasSynNodePath: Copy {
    type SynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath;
}

impl HasSynNodePath for ItemPath {
    type SynNodePath = ItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            ItemPath::Submodule(_, path) => path.syn_node_path(db).into(),
            ItemPath::MajorItem(path) => path.syn_node_path(db).into(),
            ItemPath::AssocItem(path) => path.syn_node_path(db).into(),
            ItemPath::TypeVariant(_, path) => path.syn_node_path(db).into(),
            ItemPath::ImplBlock(path) => path.syn_node_path(db).into(),
            ItemPath::Attr(_, path) => path.syn_node_path(db).into(),
        }
    }
}

#[derive(Default)]
pub(crate) struct ItemSynNodePathRegistry {
    next_disambiguators: VecPairMap<ItemPath, u8>,
}

impl ItemSynNodePathRegistry {
    fn issue_maybe_ambiguous_path<P: Copy + Into<ItemPath>>(
        &mut self,
        path: P,
    ) -> MaybeAmbiguousPath<P> {
        let next_disambiguator = self
            .next_disambiguators
            .get_value_mut_or_insert_default(path.into());
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        MaybeAmbiguousPath {
            path,
            disambiguator,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaybeAmbiguousPath<P> {
    path: P,
    disambiguator: u8,
}

impl<P> MaybeAmbiguousPath<P> {
    fn from_path(path: P) -> Self {
        Self {
            path,
            disambiguator: 0,
        }
    }

    fn unambiguous_path(self) -> Option<P> {
        (self.disambiguator == 0).then_some(self.path)
    }
}

/// this is pub(crate) because it contains AstIdx which affects incremental computation
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
#[enum_class::from_variants]
pub(crate) enum ItemSynNode {
    Submodule(SubmoduleSynNode),
    MajorItem(MajorItemSynNode),
    TypeVariant(TypeVariantSynNode),
    ImplBlock(ImplBlockSynNode),
    Attr(AttrSynNode),
}

impl ItemSynNode {
    pub(crate) fn try_new_major(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        item_path: ItemPath,
        block: DefnBlock,
    ) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, submodule_path) => Some(
                SubmoduleSynNode::new(
                    db,
                    registry,
                    submodule_path,
                    visibility,
                    ast_idx,
                    ident_token,
                )
                .into(),
            ),
            ItemPath::MajorItem(module_item_path) => Some(
                MajorItemSynNode::new(
                    db,
                    registry,
                    module_item_path,
                    visibility,
                    ast_idx,
                    ident_token,
                    block,
                )
                .into(),
            ),
            ItemPath::AssocItem(_) | ItemPath::TypeVariant(_, _) => None,
            ItemPath::ImplBlock(_) | ItemPath::Attr(_, _) => unreachable!(),
        }
    }

    pub fn syn_node_path(&self, _db: &::salsa::Db) -> ItemSynNodePath {
        match self {
            ItemSynNode::Submodule(node) => node.syn_node_path.into(),
            ItemSynNode::MajorItem(node) => node.syn_node_path.into(),
            ItemSynNode::TypeVariant(node) => node.syn_node_path.into(),
            ItemSynNode::ImplBlock(node) => node.syn_node_path().into(),
            ItemSynNode::Attr(node) => node.syn_node_path().into(),
        }
    }
}

pub trait HasAssocItemPaths: Copy {
    type AssocItemPath;

    fn assoc_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssocItemPath)];
}

pub trait HasAttrPaths: Copy {
    type AttrPath;

    fn attr_paths(self, db: &::salsa::Db) -> &[Self::AttrPath];
}
