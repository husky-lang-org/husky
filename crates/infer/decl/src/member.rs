mod field;
mod method;

pub use field::*;
pub use method::*;

use atom::symbol_proxy::Symbol;
use fold::LocalStack;
use map_collect::MapCollect;
use vec_dict::HasKey;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MemberDecl {
    AssociatedType,
    AssociatedCall,
    TypeField(Arc<FieldDecl>),
    TypeMethod(Arc<MethodDecl>),
    TraitMethod {
        trait_route: EntityRoutePtr,
        method: Arc<MethodDecl>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemberIdx(pub u8);

impl From<usize> for MemberIdx {
    fn from(raw: usize) -> Self {
        Self(raw.try_into().unwrap())
    }
}

impl MemberDecl {
    pub fn ident(&self) -> CustomIdentifier {
        match self {
            MemberDecl::AssociatedType => todo!(),
            MemberDecl::AssociatedCall => todo!(),
            MemberDecl::TypeField(field) => field.ident,
            MemberDecl::TypeMethod(method) => method.ident,
            MemberDecl::TraitMethod { method, .. } => method.ident,
        }
    }
}

impl MemberDecl {
    pub(crate) fn from_trait(
        trait_route: EntityRoutePtr,
        trait_member_impl: &TraitMemberImplDecl,
    ) -> Self {
        match trait_member_impl {
            TraitMemberImplDecl::Method(method) => MemberDecl::TraitMethod {
                trait_route,
                method: method.clone(),
            },
        }
    }
}

impl From<&TyMemberDecl> for MemberDecl {
    fn from(decl: &TyMemberDecl) -> Self {
        match decl {
            TyMemberDecl::Field(field_decl) => MemberDecl::TypeField(field_decl.clone()),
            TyMemberDecl::Method(method_decl) => MemberDecl::TypeMethod(method_decl.clone()),
            TyMemberDecl::Call => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TyMemberDecl {
    Field(Arc<FieldDecl>),
    Method(Arc<MethodDecl>),
    Call,
}

impl TyMemberDecl {
    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TyMemberDecl::Field(_) => todo!(),
            TyMemberDecl::Method(method_decl) => {
                TyMemberDecl::Method(method_decl.instantiate(instantiator))
            }
            TyMemberDecl::Call => todo!(),
        }
    }

    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        member_decl: &StaticTypeMemberDecl,
        this_ty: EntityRoutePtr,
        symbols: &[Symbol],
    ) -> Self {
        match member_decl {
            StaticTypeMemberDecl::Field => todo!(),
            StaticTypeMemberDecl::Method(method_decl) => TyMemberDecl::Method(
                MethodDecl::from_static(db, method_decl, Some(this_ty), symbols),
            ),
            StaticTypeMemberDecl::Call => todo!(),
        }
    }
}

impl HasKey<CustomIdentifier> for TyMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TyMemberDecl::Method(method_decl) => method_decl.ident,
            TyMemberDecl::Field(field_decl) => field_decl.ident,
            TyMemberDecl::Call => todo!(),
        }
    }
}

impl MemberDecl {
    pub(crate) fn collect_all(
        db: &dyn DeclQueryGroup,
        type_members: &[TyMemberDecl],
        trait_impls: &[Arc<TraiImplDecl>],
    ) -> Vec<MemberDecl> {
        let mut members: Vec<MemberDecl> = type_members.map(|decl| decl.into());
        for trait_impl in trait_impls {
            for member in trait_impl.members.iter() {
                members.push(MemberDecl::from_trait(trait_impl.trait_decl.route, member))
            }
        }
        members
    }
}

pub(crate) fn member_idx(db: &dyn DeclQueryGroup, member_route: EntityRoutePtr) -> MemberIdx {
    let this_ty = member_route.parent();
    let this_ty_decl = db.type_decl(this_ty).unwrap();
    this_ty_decl.member_idx(member_route)
}
