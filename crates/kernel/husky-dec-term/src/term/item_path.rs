use husky_entity_path::*;
use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum ItemPathDecTerm {
    Fugitive(FugitivePath),
    Trait(TraitPath),
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

impl From<ItemPathTerm> for ItemPathDecTerm {
    fn from(path: ItemPathTerm) -> Self {
        match path {
            ItemPathTerm::Fugitive(path) => ItemPathDecTerm::Fugitive(path),
            ItemPathTerm::Trait(path) => ItemPathDecTerm::Trait(path),
            ItemPathTerm::TypeOntology(path) | ItemPathTerm::TypeInstance(path) => {
                ItemPathDecTerm::Type(path)
            }
            ItemPathTerm::TypeVariant(path) => ItemPathDecTerm::TypeVariant(path),
        }
    }
}

impl From<ItemPathTerm> for DecTerm {
    fn from(path: ItemPathTerm) -> Self {
        DecTerm::EntityPath(path.into())
    }
}

impl ItemPathDecTerm {
    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            ItemPathDecTerm::Type(path) => Some(path),
            ItemPathDecTerm::Fugitive(_)
            | ItemPathDecTerm::Trait(_)
            | ItemPathDecTerm::TypeVariant(_) => None,
        }
    }
}
impl From<FugitivePath> for DecTerm {
    fn from(value: FugitivePath) -> Self {
        DecTerm::EntityPath(value.into())
    }
}

impl From<TraitPath> for DecTerm {
    fn from(value: TraitPath) -> Self {
        DecTerm::EntityPath(value.into())
    }
}

impl From<TypePath> for DecTerm {
    fn from(value: TypePath) -> Self {
        DecTerm::EntityPath(value.into())
    }
}

impl ItemPathDecTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        match self {
            ItemPathDecTerm::Fugitive(path) => path.display_with_db_fmt(f, db),
            ItemPathDecTerm::Trait(path) => path.display_with_db_fmt(f, db),
            ItemPathDecTerm::Type(path) => path.display_with_db_fmt(f, db),
            ItemPathDecTerm::TypeVariant(path) => path.display_with_db_fmt(f, db),
        }
    }
}