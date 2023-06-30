mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub enum TypeDeclarativeSignature {
    Enum(EnumTypeDeclarativeSignature),
    PropsStruct(PropsStructDeclarativeSignature),
    UnitStruct(UnitStructTypeDeclarativeSignature),
    TupleStruct(TupleStructTypeDeclarativeSignature),
    Record(RecordTypeDeclarativeSignature),
    Inductive(InductiveTypeDeclarativeSignature),
    Structure(StructureTypeDeclarativeSignature),
    Extern(ExternTypeDeclarativeSignature),
    Union(UnionTypeDeclarativeSignature),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeDeclarativeSignatureTemplate {
    Enum(EnumDeclarativeSignatureTemplate),
    PropsStruct(PropsStructDeclarativeSignatureTemplate),
    UnitStruct(UnitStructDeclarativeSignatureTemplate),
    TupleStruct(TupleStructDeclarativeSignatureTemplate),
    Record(RecordDeclarativeSignatureTemplate),
    Inductive(InductiveDeclarativeSignatureTemplate),
    Structure(StructureDeclarativeSignatureTemplate),
    Extern(ExternDeclarativeSignatureTemplate),
    Union(UnionDeclarativeSignatureTemplate),
}

impl TypeDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterDeclarativeSignature] {
        match self {
            TypeDeclarativeSignatureTemplate::Enum(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::PropsStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Record(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Inductive(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Structure(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Extern(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Union(decl) => decl.implicit_parameters(db),
        }
    }
}

impl HasDeclarativeSignatureTemplate for TypePath {
    type DeclarativeSignatureTemplate = TypeDeclarativeSignatureTemplate;

    #[inline(always)]
    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TypePath,
) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    Ok(match decl {
        TypeDecl::Enum(decl) => EnumDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into(),
        TypeDecl::PropsStruct(decl) => {
            PropsStructDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::UnitStruct(decl) => {
            UnitStructDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::TupleStruct(decl) => {
            TupleStructDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::Record(decl) => {
            RecordDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::Inductive(decl) => {
            InductiveDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::Structure(decl) => {
            StructureDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::Extern(decl) => {
            ExternDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeDecl::Union(decl) => {
            UnionDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
    })
}

fn construct_self_ty(
    db: &dyn DeclarativeSignatureDb,
    path: TypePath,
    implicit_parameters: &[ImplicitParameterDeclarativeSignature],
) -> DeclarativeTerm {
    let mut self_ty: DeclarativeTerm = path.into();
    for implicit_parameter in implicit_parameters {
        self_ty =
            DeclarativeTermExplicitApplication::new(db, self_ty, implicit_parameter.symbol().into())
                .into()
    }
    self_ty
}
