use crate::*;
use infer_error::derived;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LazyQualifiedTy {
    pub qual: LazyQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for LazyQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        write!(result, "{: <12?} {:?}", self.qual, self.ty).unwrap()
    }
}

impl LazyQualifiedTy {
    pub(crate) fn ty_ty() -> Self {
        Self {
            qual: LazyQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }

    pub(crate) fn trait_ty() -> Self {
        Self {
            qual: LazyQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TraitType),
        }
    }

    pub(crate) fn member_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: LazyQualifier,
        field_ty: EntityRoutePtr,
        field_liason: MemberLiason,
    ) -> InferResult<Self> {
        Ok(Self::new(
            LazyQualifier::member_lazy_qualifier(
                this_qual,
                field_liason,
                db.is_copyable(field_ty)?,
            )?,
            field_ty,
        ))
    }

    pub(crate) fn parameter_lazy_qualified_ty(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterLiason,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        Ok(LazyQualifiedTy::new(
            LazyQualifier::parameter(parameter_liason, db.is_copyable(ty)?),
            ty,
        ))
    }

    pub fn new(qual: LazyQualifier, ty: EntityRoutePtr) -> Self {
        emsg_once!("handle ref");
        Self { qual, ty }
    }

    pub(crate) fn use_for_init(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let | InitKind::Var => Err(derived!(
                "let or var is not allowed in lazy context".to_string()
            ))?,
            InitKind::Decl => match self.qual {
                LazyQualifier::Copyable => LazyQualifier::Copyable,
                LazyQualifier::PureRef => LazyQualifier::PureRef,
                LazyQualifier::EvalRef | LazyQualifier::Transient => LazyQualifier::EvalRef,
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub(crate) fn is_implicitly_convertible_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_liason: OutputLiason,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.ty, output_ty) {
            return false;
        }
        match output_liason {
            OutputLiason::Transfer => match self.qual {
                LazyQualifier::Copyable => true,
                LazyQualifier::PureRef => todo!(),
                LazyQualifier::EvalRef => todo!(),
                LazyQualifier::Transient => true,
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LazyQualifier {
    Copyable,
    PureRef,
    EvalRef,
    Transient,
}

impl LazyQualifier {
    pub fn feature(is_copyable: bool) -> LazyQualifier {
        if is_copyable {
            LazyQualifier::Copyable
        } else {
            LazyQualifier::EvalRef
        }
    }

    pub fn binding(self, contract: LazyContract) -> Binding {
        match self {
            LazyQualifier::PureRef => match contract {
                LazyContract::Pure => Binding::TempRef,
                LazyContract::EvalRef => todo!(),
                LazyContract::Move => todo!(),
                LazyContract::Pass => Binding::TempRef,
            },
            LazyQualifier::Transient => todo!(),
            LazyQualifier::Copyable => Binding::Copy,
            LazyQualifier::EvalRef => Binding::EvalRef,
        }
    }

    pub fn variable_use(self, contract: LazyContract) -> InferResult<Self> {
        Ok(match self {
            LazyQualifier::Copyable => match contract {
                LazyContract::Pass => LazyQualifier::Copyable,
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyQualifier::Copyable,
                LazyContract::Move => todo!(),
            },
            LazyQualifier::PureRef => match contract {
                LazyContract::Pass => todo!(),
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyQualifier::EvalRef => match contract {
                LazyContract::Pass => LazyQualifier::EvalRef,
                LazyContract::EvalRef => LazyQualifier::EvalRef,
                LazyContract::Pure => LazyQualifier::PureRef,
                LazyContract::Move => todo!(),
            },
            LazyQualifier::Transient => todo!(),
        })
    }

    pub fn member_lazy_qualifier(
        this_qual: LazyQualifier,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        Ok(if is_field_copyable {
            LazyQualifier::Copyable
        } else {
            // non-copyable
            this_qual
        })
    }

    pub fn parameter_use_lazy_qualifier(
        input_liason: ParameterLiason,
        is_copyable: bool,
        contract: LazyContract,
    ) -> InferResult<Self> {
        Self::parameter(input_liason, is_copyable).variable_use(contract)
    }

    pub fn parameter(parameter_liason: ParameterLiason, is_copyable: bool) -> Self {
        match parameter_liason {
            ParameterLiason::Pure => {
                if is_copyable {
                    LazyQualifier::Copyable
                } else {
                    LazyQualifier::PureRef
                }
            }
            ParameterLiason::EvalRef => LazyQualifier::EvalRef,
            ParameterLiason::Move => todo!(),
            ParameterLiason::TempRefMut => todo!(),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::TempRef => todo!(),
        }
    }

    pub fn method_opt_output_binding(
        self,
        output_liason: OutputLiason,
        output_contract: LazyContract,
        is_output_ty_copyable: bool,
    ) -> Option<Binding> {
        match output_liason {
            OutputLiason::Transfer => None,
            OutputLiason::MemberAccess { member_liason } => {
                Some(self.member_binding(member_liason, output_contract, is_output_ty_copyable))
            }
        }
    }

    pub fn member_binding(
        self,
        member_liason: MemberLiason,
        member_contract: LazyContract,
        is_member_ty_copyable: bool,
    ) -> Binding {
        if is_member_ty_copyable {
            match member_contract {
                LazyContract::Pass => Binding::Copy,
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => Binding::Copy,
                LazyContract::Move => todo!(),
            }
        } else {
            // non-copyable
            match member_contract {
                LazyContract::Pass => match self {
                    LazyQualifier::Copyable => todo!(),
                    LazyQualifier::PureRef => Binding::TempRef,
                    LazyQualifier::EvalRef => Binding::EvalRef,
                    LazyQualifier::Transient => Binding::Move,
                },
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => todo!(),
                LazyContract::Move => todo!(),
            }
        }
    }
}
