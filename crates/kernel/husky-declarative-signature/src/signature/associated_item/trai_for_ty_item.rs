mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemDeclarativeSignatureTemplate {
    AssociatedFn(TraitForTypeAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TraitForTypeMethodFnDeclarativeSignatureTemplate),
    AssociatedType(TraitForTypeAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedVal(TraitForTypeAssociatedValDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for TraitForTypeItemPath {
    type DeclarativeSignatureTemplate = TraitForTypeItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_for_ty_item_syn_declarative_signature_from_decl(db, self)
    }
}

// #[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_item_syn_declarative_signature_from_decl(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> DeclarativeSignatureResult<TraitForTypeItemDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TraitForTypeItemSynDecl::AssociatedFn(decl) => {
            TraitForTypeAssociatedFnDeclarativeSignatureTemplate::from_decl(db, decl)
                .map(Into::into)
        }
        TraitForTypeItemSynDecl::MethodFn(decl) => {
            TraitForTypeMethodFnDeclarativeSignatureTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeDeclarativeSignatureTemplate::from_decl(db, decl)
                .map(Into::into)
        }
        TraitForTypeItemSynDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValDeclarativeSignatureTemplate::from_decl(db, decl)
                .map(Into::into)
        }
    }
}

impl TraitForTypeItemDeclarativeSignatureTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedFn(tmpl) => {
                tmpl.template_parameters(db)
            }
            TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(tmpl) => {
                tmpl.template_parameters(db)
            }
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(tmpl) => {
                tmpl.template_parameters(db)
            }
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedVal(tmpl) => &[],
        }
    }
}
