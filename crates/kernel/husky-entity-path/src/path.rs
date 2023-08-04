mod associated_item;
mod impl_block;
mod module_item;
mod variant;

pub use self::associated_item::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum EntityPath {
    Module(ModulePath),
    MajorItem(MajarItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(TypeVariantPath),
    ImplBlock(ImplBlockPath),
}

impl EntityPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        match self {
            EntityPath::Module(path) => path.ident(db),
            EntityPath::MajorItem(path) => path.ident(db),
            EntityPath::AssociatedItem(path) => path.ident(db),
            EntityPath::TypeVariant(path) => path.ident(db),
            EntityPath::ImplBlock(_) => todo!(),
        }
    }

    pub fn module_item_path(self) -> Option<MajarItemPath> {
        match self {
            EntityPath::MajorItem(module_item_path) => Some(module_item_path),
            _ => None,
        }
    }

    pub fn ty_path(self) -> Option<TypePath> {
        self.module_item_path()?.ty_path()
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            EntityPath::Module(path) => path,
            EntityPath::MajorItem(path) => path.module_path(db),
            EntityPath::AssociatedItem(path) => path.module_path(db),
            EntityPath::TypeVariant(path) => path.module_path(db),
            EntityPath::ImplBlock(path) => path.module_path(db),
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            EntityPath::Module(_path) => EntityKind::Module,
            EntityPath::MajorItem(path) => path.item_kind(db),
            EntityPath::AssociatedItem(path) => path.item_kind(db),
            EntityPath::TypeVariant(_) => EntityKind::TypeVariant,
            EntityPath::ImplBlock(_) => EntityKind::ImplBlock,
        }
    }

    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            EntityPath::Module(path) => Some(path.into()),
            EntityPath::MajorItem(path) => Some(path.into()),
            EntityPath::AssociatedItem(_)
            | EntityPath::TypeVariant(_)
            | EntityPath::ImplBlock(_) => None,
        }
    }
}

impl From<FugitivePath> for EntityPath {
    fn from(v: FugitivePath) -> Self {
        EntityPath::MajorItem(v.into())
    }
}

impl From<TypePath> for EntityPath {
    fn from(v: TypePath) -> Self {
        EntityPath::MajorItem(v.into())
    }
}

impl From<TraitPath> for EntityPath {
    fn from(v: TraitPath) -> Self {
        EntityPath::MajorItem(v.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum ItemPath {
    Submodule(SubmodulePath),
    MajorItem(MajarItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(TypeVariantPath),
    ImplBlock(ImplBlockPath),
}

impl ItemPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        match self {
            ItemPath::Submodule(path) => path.ident(db),
            ItemPath::MajorItem(path) => path.ident(db),
            ItemPath::AssociatedItem(path) => path.ident(db),
            ItemPath::TypeVariant(path) => path.ident(db),
            ItemPath::ImplBlock(_) => todo!(),
        }
    }

    pub fn module_item_path(self) -> Option<MajarItemPath> {
        match self {
            ItemPath::MajorItem(module_item_path) => Some(module_item_path),
            _ => None,
        }
    }

    pub fn ty_path(self) -> Option<TypePath> {
        self.module_item_path()?.ty_path()
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            ItemPath::Submodule(path) => path.inner(),
            ItemPath::MajorItem(path) => path.module_path(db),
            ItemPath::AssociatedItem(path) => path.module_path(db),
            ItemPath::TypeVariant(path) => path.module_path(db),
            ItemPath::ImplBlock(path) => path.module_path(db),
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            ItemPath::Submodule(_path) => EntityKind::Module,
            ItemPath::MajorItem(path) => path.item_kind(db),
            ItemPath::AssociatedItem(path) => path.item_kind(db),
            ItemPath::TypeVariant(_) => EntityKind::TypeVariant,
            ItemPath::ImplBlock(_) => EntityKind::ImplBlock,
        }
    }

    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            ItemPath::Submodule(path) => Some(path.inner().into()),
            ItemPath::MajorItem(path) => Some(path.into()),
            ItemPath::AssociatedItem(_) | ItemPath::TypeVariant(_) | ItemPath::ImplBlock(_) => None,
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for ItemPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        match self {
            ItemPath::Submodule(path) => path.display_with_db_fmt(f, db, level),
            ItemPath::MajorItem(path) => path.display_with_db_fmt(f, db, level),
            ItemPath::AssociatedItem(path) => path.display_with_db_fmt(f, db, level),
            ItemPath::TypeVariant(path) => path.display_with_db_fmt(f, db, level),
            ItemPath::ImplBlock(_) => todo!(),
        }
    }
}

impl From<FugitivePath> for ItemPath {
    fn from(v: FugitivePath) -> Self {
        ItemPath::MajorItem(v.into())
    }
}

impl From<TypePath> for ItemPath {
    fn from(v: TypePath) -> Self {
        ItemPath::MajorItem(v.into())
    }
}

impl From<TraitPath> for ItemPath {
    fn from(v: TraitPath) -> Self {
        ItemPath::MajorItem(v.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum MajorEntityPath {
    Module(ModulePath),
    MajorItem(MajarItemPath),
}

impl From<FugitivePath> for MajorEntityPath {
    fn from(v: FugitivePath) -> Self {
        MajorEntityPath::MajorItem(v.into())
    }
}

impl From<TypePath> for MajorEntityPath {
    fn from(v: TypePath) -> Self {
        MajorEntityPath::MajorItem(v.into())
    }
}

impl From<TraitPath> for MajorEntityPath {
    fn from(v: TraitPath) -> Self {
        MajorEntityPath::MajorItem(v.into())
    }
}

impl MajorEntityPath {
    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        match self {
            MajorEntityPath::Module(path) => path.crate_path(db),
            MajorEntityPath::MajorItem(path) => path.crate_path(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum PrincipalEntityPath {
    Module(ModulePath),
    MajorItem(MajarItemPath),
    TypeVariant(TypeVariantPath),
}

impl PrincipalEntityPath {
    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            PrincipalEntityPath::Module(path) => Some(path.into()),
            PrincipalEntityPath::MajorItem(path) => Some(path.into()),
            PrincipalEntityPath::TypeVariant(_) => None,
        }
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            PrincipalEntityPath::Module(_path) => EntityKind::Module,
            PrincipalEntityPath::MajorItem(path) => path.item_kind(db),
            PrincipalEntityPath::TypeVariant(_) => EntityKind::TypeVariant,
        }
    }
}

impl From<MajorEntityPath> for PrincipalEntityPath {
    fn from(path: MajorEntityPath) -> Self {
        match path {
            MajorEntityPath::Module(path) => todo!(),
            // path.into(),
            MajorEntityPath::MajorItem(path) => path.into(),
        }
    }
}

impl From<PrincipalEntityPath> for EntityPath {
    fn from(path: PrincipalEntityPath) -> Self {
        match path {
            PrincipalEntityPath::Module(path) => path.into(),
            PrincipalEntityPath::MajorItem(path) => path.into(),
            PrincipalEntityPath::TypeVariant(path) => path.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum PrincipalItemPath {
    Submodule(SubmodulePath),
    MajorItem(MajarItemPath),
    TypeVariant(TypeVariantPath),
}
