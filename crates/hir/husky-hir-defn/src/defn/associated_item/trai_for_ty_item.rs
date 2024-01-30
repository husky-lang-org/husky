mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

use husky_hir_decl::decl::TraitForTypeItemHirDecl;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDefn {
    AssociatedFn(TraitForTypeAssociatedFnHirDefn),
    MethodFn(TraitForTypeMethodFnHirDefn),
    AssociatedType(TraitForTypeAssociatedTypeHirDefn),
    AssociatedVal(TraitForTypeAssociatedValHirDefn),
}

impl From<TraitForTypeItemHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeItemHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TraitForTypeItemHirDefn {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.path(db),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> TraitForTypeItemHirDecl {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.hir_decl(db).into(),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(slf) => {
                slf.hir_eager_expr_region(db).map(Into::into)
            }
            TraitForTypeItemHirDefn::MethodFn(slf) => slf.hir_eager_expr_region(db).map(Into::into),
            TraitForTypeItemHirDefn::AssociatedType(_slf) => None,
            TraitForTypeItemHirDefn::AssociatedVal(slf) => slf.hir_expr_region(db),
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.dependencies(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.dependencies(db),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.dependencies(db),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            TraitForTypeItemHirDefn::AssociatedFn(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::MethodFn(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::AssociatedType(hir_defn) => hir_defn.version_stamp(db),
            TraitForTypeItemHirDefn::AssociatedVal(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TraitForTypeItemPath {
    type HirDefn = TraitForTypeItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        trai_for_ty_item_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_for_ty_item_hir_defn(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> Option<TraitForTypeItemHirDefn> {
    match path.hir_decl(db)? {
        TraitForTypeItemHirDecl::AssociatedFn(hir_decl) => {
            Some(TraitForTypeAssociatedFnHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::MethodFn(hir_decl) => {
            Some(TraitForTypeMethodFnHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::AssociatedType(hir_decl) => {
            Some(TraitForTypeAssociatedTypeHirDefn::new(db, path, hir_decl).into())
        }
        TraitForTypeItemHirDecl::AssociatedVal(hir_decl) => {
            Some(TraitForTypeAssociatedValHirDefn::new(db, path, hir_decl).into())
        }
    }
}
