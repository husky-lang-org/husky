use crate::*;
use husky_ty_expectation::TypePathDisambiguation;
use husky_vfs::Toolchain;

pub trait HasType: Copy {
    fn ty(self, db: &dyn TypeDb) -> TermResult<EtherealTerm>;
}

pub trait HasTypeGivenToolchain: Copy {
    fn ty(self, db: &dyn TypeDb, toolchain: Toolchain) -> TermResult<EtherealTerm>;
}

pub trait HasTypeGivenDisambiguation: Copy {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm>;
}

impl HasTypeGivenDisambiguation for EntityPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        match self {
            EntityPath::Module(path) => Ok(db.term_menu(path.toolchain(db)).module_ty_ontology()),
            EntityPath::ModuleItem(path) => path.ty(db, disambiguation),
            EntityPath::AssociatedItem(path) => path.ty(db, disambiguation),
            EntityPath::TypeVariant(path) => path.ty(db, disambiguation),
        }
    }
}

impl HasTypeGivenDisambiguation for ModuleItemPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        match self {
            ModuleItemPath::Type(path) => path.ty(db, disambiguation),
            ModuleItemPath::Trait(path) => path.ty(db),
            ModuleItemPath::Form(path) => path.ty(db),
        }
    }
}

impl HasType for TraitPath {
    fn ty(self, db: &dyn TypeDb) -> TermResult<EtherealTerm> {
        trai_path_ty(db, self)
    }
}

impl HasType for FormPath {
    fn ty(self, db: &dyn TypeDb) -> TermResult<EtherealTerm> {
        form_path_ty(db, self)
    }
}

impl HasTypeGivenDisambiguation for TypePath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        ty_path_ty(db, self, disambiguation)
    }
}

impl HasTypeGivenDisambiguation for AssociatedItemPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        match self {
            AssociatedItemPath::TypeItem(path) => path.ty(db, disambiguation),
            AssociatedItemPath::TraitItem(path) => path.ty(db, disambiguation),
            AssociatedItemPath::TraitForTypeItem(path) => path.ty(db, disambiguation),
        }
    }
}

impl HasTypeGivenDisambiguation for TypeItemPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        ty_item_path_ty(db, self)
    }
}

impl HasTypeGivenDisambiguation for TraitItemPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        todo!()
    }
}

impl HasTypeGivenDisambiguation for TraitForTypeItemPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        todo!()
    }
}

impl HasTypeGivenDisambiguation for TypeVariantPath {
    fn ty(
        self,
        db: &dyn TypeDb,
        disambiguation: TypePathDisambiguation,
    ) -> TermResult<EtherealTerm> {
        ty_variant_path_ty(db, self)
    }
}

impl HasTypeGivenToolchain for EtherealTerm {
    fn ty(self, db: &dyn TypeDb, toolchain: Toolchain) -> TermResult<EtherealTerm> {
        todo!()
        // self.ty_unchecked(db)?.checked(db)
    }
}
