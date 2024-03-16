use super::*;

#[salsa::as_id]
#[salsa::deref_id]
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitForTypeItemPath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitForTypeItemPathData {
    pub impl_block: TraitForTypeImplBlockPath,
    pub ident: Ident,
    pub item_kind: TraitItemKind,
}

impl From<TraitForTypeItemPath> for ItemPath {
    fn from(path: TraitForTypeItemPath) -> Self {
        ItemPath::AssocItem(path.into())
    }
}

impl TraitForTypeItemPath {
    pub fn new(
        impl_block: TraitForTypeImplBlockPath,
        ident: Ident,
        item_kind: TraitItemKind,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::AssocItem(AssocItemPathData::TraitForTypeItem(
                TraitForTypeItemPathData {
                    impl_block,
                    ident,
                    item_kind,
                },
            )),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TraitForTypeItemPathData {
        match self.0.data(db) {
            ItemPathData::AssocItem(AssocItemPathData::TraitForTypeItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn impl_block(self, db: &::salsa::Db) -> TraitForTypeImplBlockPath {
        self.data(db).impl_block
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.data(db).item_kind
    }

    #[inline(never)]
    fn show_aux(self, _f: &mut std::fmt::Formatter<'_>, _db: &::salsa::Db) -> std::fmt::Result {
        todo!()
    }
}

impl TraitForTypeItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TraitForTypeItemPath {
        TraitForTypeItemPath(id)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.impl_block.module_path(db)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.impl_block.toolchain(db)
    }

    pub fn entity_kind(self, _db: &::salsa::Db) -> EntityKind {
        EntityKind::AssocItem {
            assoc_item_kind: AssocItemKind::TraitForTypeItem(self.item_kind),
        }
    }
}

impl salsa::DisplayWithDb for TraitForTypeItemPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}
