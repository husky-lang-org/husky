pub mod r#mut;
pub mod option;
pub mod owned;
pub mod primitive;
pub mod r#ref;
mod ritchie;
mod static_ref;
mod tuple;
pub mod vec;

use std::{cmp::Ordering, convert::Infallible};

use super::*;
use crate::{
    exception::Excepted,
    frozen::{r#mut::FrozenMut, Frozen, FrozenDyn},
    slush::{SlushValue, SlushValues},
    *,
};
use frozen::FrozenValue;
use husky_decl_macro_utils::{
    for_all_non_unit_tuple_tys, for_all_primitive_tys, for_all_ritchie_tys,
};
use husky_value::{vm_control_flow::VmControlFlow, IsThawedValue};
use husky_value_macros::thawed_value_ty;
use husky_value_protocol::presentation::{EnumUnitValuePresenter, ValuePresentation};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use owned::OwnedThawedValue;

/// Slush is the static version of a type
pub trait Thawed: Sized + std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type Frozen: Frozen<Thawed = Self>;
    fn freeze(&self) -> Self::Frozen;

    fn is_copyable() -> bool;

    /// copy if the type is copyable
    ///
    /// note that it should always be either some or none for a fixed type
    fn try_copy_thawed(&self) -> Option<ThawedValue>;

    fn is_some(&self) -> bool {
        panic!("type `{}` is not an Option", std::any::type_name::<Self>())
    }

    fn is_none(&self) -> bool {
        panic!("type `{}` is not an Option", std::any::type_name::<Self>())
    }

    fn index_owned_thawed(self, index: usize) -> ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support indexing owned",
            std::any::type_name::<Self>()
        )
    }

    fn index_ref_thawed<'a>(&'a self, index: usize) -> ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support indexing ref",
            std::any::type_name::<Self>()
        )
    }

    fn index_leash_thawed(&'static self, index: usize) -> ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support indexing leash",
            std::any::type_name::<Self>()
        )
    }

    fn unwrap_ref_thawed<'a>(&'a self) -> ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support unwrap",
            std::any::type_name::<Self>()
        )
    }

    fn unwrap_leash_thawed(&'static self) -> ExceptedThawedValue {
        panic!(
            "type `{}` doesn't support unwrap",
            std::any::type_name::<Self>()
        )
    }
}

pub trait ThawedDyn:
    std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe + 'static
{
    fn freeze(&self) -> Arc<dyn FrozenDyn>;

    fn type_name_dyn(&self) -> &'static str;

    fn is_some_dyn(&self) -> bool;

    fn is_none_dyn(&self) -> bool;

    fn index_owned_thawed_dyn(self: Box<Self>, index: usize) -> ExceptedThawedValue;
    fn index_ref_thawed_dyn<'a>(&'a self, index: usize) -> ExceptedThawedValue;
    fn index_leash_thawed_dyn(&'static self, index: usize) -> ExceptedThawedValue;

    // todo: unwrap owned
    fn unwrap_ref_thawed_dyn<'a>(&'a self) -> ExceptedThawedValue;
    fn unwrap_leash_thawed_dyn(&'static self) -> ExceptedThawedValue;

    fn try_copy_thawed_dyn(&self) -> Option<ThawedValue>;
}

impl<T> ThawedDyn for T
where
    T: Thawed,
{
    fn freeze(&self) -> Arc<dyn FrozenDyn> {
        Arc::new(self.freeze())
    }

    fn type_name_dyn(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn is_some_dyn(&self) -> bool {
        self.is_some()
    }

    fn is_none_dyn(&self) -> bool {
        self.is_none()
    }

    fn index_owned_thawed_dyn(self: Box<Self>, index: usize) -> ExceptedThawedValue {
        self.index_owned_thawed(index)
    }

    fn index_ref_thawed_dyn<'a>(&'a self, index: usize) -> ExceptedThawedValue {
        self.index_ref_thawed(index)
    }

    fn index_leash_thawed_dyn(&'static self, index: usize) -> ExceptedThawedValue {
        self.index_leash_thawed(index)
    }

    fn unwrap_ref_thawed_dyn<'a>(&'a self) -> ExceptedThawedValue {
        T::unwrap_ref_thawed(self)
    }

    fn unwrap_leash_thawed_dyn(&'static self) -> ExceptedThawedValue {
        T::unwrap_leash_thawed(self)
    }

    fn try_copy_thawed_dyn(&self) -> Option<ThawedValue> {
        self.try_copy_thawed()
    }
}

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[thawed_value_ty]
#[derive(Debug)]
#[repr(u8)]
pub enum ThawedValue {
    Uninit,
    Invalid,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    /// `Box<T>`
    Owned(OwnedThawedValue),
    // ad hoc
    /// `~T`
    Leash(&'static dyn ThawedDyn),
    /// `&T` for T Sized
    Ref(*const dyn ThawedDyn),
    /// `&mut T` for T Sized
    Mut(*mut dyn ThawedDyn),
    OptionBox(Option<Box<dyn ThawedDyn>>),
    OptionLeash(Option<&'static dyn ThawedDyn>),
    OptionSizedRef(Option<*const dyn ThawedDyn>),
    OptionSizedMut(Option<*mut dyn ThawedDyn>),
    EnumUnit {
        index: usize,
        presenter: EnumUnitValuePresenter,
    },
}

pub type ExceptedThawedValue = Excepted<ThawedValue>;

impl IsThawedValue for ThawedValue {
    type Value = Value;

    fn new_uninit() -> Self {
        ThawedValue::Uninit
    }

    fn from_r8(r: u8) -> Self {
        ThawedValue::R8(r)
    }

    fn from_r16(r: u16) -> Self {
        ThawedValue::R16(r)
    }

    fn from_r32(r: u32) -> Self {
        ThawedValue::R32(r)
    }

    fn from_r64(r: u64) -> Self {
        ThawedValue::R64(r)
    }

    fn from_r128(r: u128) -> Self {
        ThawedValue::R128(r)
    }

    fn from_rsize(r: u64) -> Self {
        ThawedValue::RSize(r as usize)
    }

    fn r#move(&mut self) -> Self {
        std::mem::replace(self, ThawedValue::Moved)
    }

    fn from_str_literal(str_value: Arc<str>) -> Self {
        todo!()
    }

    fn from_enum_index(index: usize, presenter: EnumUnitValuePresenter) -> Self {
        todo!()
    }

    fn to_bool(self) -> bool {
        todo!()
    }

    fn to_usize(self) -> usize {
        todo!()
    }

    fn is_none(self) -> bool {
        todo!()
    }

    fn is_some(self) -> bool {
        todo!()
    }

    fn index(self, index: usize) -> Result<Self, <Self::Value as husky_value::IsValue>::Exception> {
        todo!()
    }

    fn unwrap(self) -> Result<Self, <Self::Value as husky_value::IsValue>::Exception> {
        todo!()
    }

    fn present(
        &self,
        value_presenter_cache: &mut husky_value_protocol::presentation::ValuePresenterCache,
        value_presentation_synchrotron: &mut husky_value_protocol::presentation::synchrotron::ValuePresentationSynchrotron,
    ) -> ValuePresentation {
        todo!()
    }

    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }

    fn freeze(&self) -> <Self::Value as husky_value::IsValue>::FrozenValue {
        match *self {
            ThawedValue::Uninit => todo!(),
            ThawedValue::Invalid => todo!(),
            ThawedValue::Moved => todo!(),
            ThawedValue::Unit(()) => FrozenValue::Unit(()),
            ThawedValue::Bool(val) => FrozenValue::Bool(val),
            ThawedValue::Char(val) => FrozenValue::Char(val),
            ThawedValue::I8(val) => FrozenValue::I8(val),
            ThawedValue::I16(val) => FrozenValue::I16(val),
            ThawedValue::I32(val) => FrozenValue::I32(val),
            ThawedValue::I64(val) => FrozenValue::I64(val),
            ThawedValue::I128(val) => FrozenValue::I128(val),
            ThawedValue::ISize(val) => FrozenValue::ISize(val),
            ThawedValue::U8(val) => FrozenValue::U8(val),
            ThawedValue::U16(val) => FrozenValue::U16(val),
            ThawedValue::U32(val) => FrozenValue::U32(val),
            ThawedValue::U64(val) => FrozenValue::U64(val),
            ThawedValue::U128(val) => FrozenValue::U128(val),
            ThawedValue::USize(val) => FrozenValue::USize(val),
            ThawedValue::R8(val) => FrozenValue::R8(val),
            ThawedValue::R16(val) => FrozenValue::R16(val),
            ThawedValue::R32(val) => FrozenValue::R32(val),
            ThawedValue::R64(val) => FrozenValue::R64(val),
            ThawedValue::R128(val) => FrozenValue::R128(val),
            ThawedValue::RSize(val) => FrozenValue::RSize(val),
            ThawedValue::F32(val) => FrozenValue::F32(val),
            ThawedValue::F64(val) => FrozenValue::F64(val),
            ThawedValue::StringLiteral(id) => FrozenValue::StringLiteral(id),
            ThawedValue::EnumUnit { index, presenter } => {
                FrozenValue::EnumUsize { index, presenter }
            }
            ThawedValue::Owned(ref slf) => FrozenValue::Owned(slf.freeze()),
            ThawedValue::Leash(_) => todo!(),
            ThawedValue::Ref(_) => todo!(),
            ThawedValue::Mut(_) => todo!(),
            ThawedValue::OptionBox(_) => todo!(),
            ThawedValue::OptionLeash(_) => todo!(),
            ThawedValue::OptionSizedRef(_) => todo!(),
            ThawedValue::OptionSizedMut(_) => todo!(),
        }
    }
}

impl ThawedValue {
    pub fn from_owned<T>(t: T) -> Self
    where
        T: Thawed,
    {
        ThawedValue::Owned(OwnedThawedValue::upcast_from_owned(t))
    }

    pub fn into_owned<T>(self) -> T {
        todo!()
    }

    pub fn from_ref<'a, T>(t: &'a T) -> Self {
        todo!()
    }

    pub fn from_leash<T>(t: &'static T) -> Self {
        todo!()
    }

    pub fn into_leash<T>(self) -> &'static T {
        todo!()
    }

    pub fn into_ref<'a, T>(self, slush_values: Option<&mut SlushValues>) -> &'a T
    where
        T: Boiled,
    {
        match self {
            ThawedValue::Uninit => todo!(),
            ThawedValue::Invalid => todo!(),
            ThawedValue::Moved => todo!(),
            ThawedValue::Unit(_) => todo!(),
            ThawedValue::Bool(_) => todo!(),
            ThawedValue::Char(_) => todo!(),
            ThawedValue::I8(_) => todo!(),
            ThawedValue::I16(_) => todo!(),
            ThawedValue::I32(_) => todo!(),
            ThawedValue::I64(_) => todo!(),
            ThawedValue::I128(_) => todo!(),
            ThawedValue::ISize(_) => todo!(),
            ThawedValue::U8(_) => todo!(),
            ThawedValue::U16(_) => todo!(),
            ThawedValue::U32(_) => todo!(),
            ThawedValue::U64(_) => todo!(),
            ThawedValue::U128(_) => todo!(),
            ThawedValue::USize(_) => todo!(),
            ThawedValue::R8(_) => todo!(),
            ThawedValue::R16(_) => todo!(),
            ThawedValue::R32(_) => todo!(),
            ThawedValue::R64(_) => todo!(),
            ThawedValue::R128(_) => todo!(),
            ThawedValue::RSize(_) => todo!(),
            ThawedValue::F32(_) => todo!(),
            ThawedValue::F64(_) => todo!(),
            ThawedValue::StringLiteral(_) => todo!(),
            ThawedValue::Owned(slf) => {
                // todo: make the whole function unsafe
                let t: &T = slf.downcast_as_ref();
                let t = unsafe { std::mem::transmute(t) };
                slush_values
                    .unwrap()
                    .push(SlushValue::Box(slf.into_inner()));
                t
            }
            ThawedValue::Leash(slf) => {
                let slf: &<T as Boiled>::Thawed = ((slf as &dyn ThawedDyn) as &dyn std::any::Any)
                    .downcast_ref()
                    .expect("type id is correct");
                unsafe { std::mem::transmute(slf) }
            }
            ThawedValue::Ref(_) => todo!(),
            ThawedValue::Mut(_) => todo!(),
            ThawedValue::OptionBox(_) => todo!(),
            ThawedValue::OptionLeash(_) => todo!(),
            ThawedValue::OptionSizedRef(_) => todo!(),
            ThawedValue::OptionSizedMut(_) => todo!(),
            ThawedValue::EnumUnit { .. } => todo!(),
        }
    }
}

impl PartialEq for ThawedValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit(l0), Self::Unit(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::I128(l0), Self::I128(r0)) => l0 == r0,
            (Self::ISize(l0), Self::ISize(r0)) => l0 == r0,
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::U128(l0), Self::U128(r0)) => l0 == r0,
            (Self::USize(l0), Self::USize(r0)) => l0 == r0,
            (Self::R8(l0), Self::R8(r0)) => l0 == r0,
            (Self::R16(l0), Self::R16(r0)) => l0 == r0,
            (Self::R32(l0), Self::R32(r0)) => l0 == r0,
            (Self::R64(l0), Self::R64(r0)) => l0 == r0,
            (Self::R128(l0), Self::R128(r0)) => l0 == r0,
            (Self::RSize(l0), Self::RSize(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::StringLiteral(l0), Self::StringLiteral(r0)) => todo!(),
            (Self::Owned(l0), Self::Owned(r0)) => todo!(),
            (Self::Leash(l0), Self::Leash(r0)) => todo!(),
            (Self::OptionBox(l0), Self::OptionBox(r0)) => todo!(),
            (Self::OptionLeash(l0), Self::OptionLeash(r0)) => todo!(),
            (Self::EnumUnit { index: l0, .. }, Self::EnumUnit { index: r0, .. }) => l0 == r0,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for ThawedValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use ThawedValue::*;
        match (self, other) {
            (Unit(_), Unit(_)) => Some(Ordering::Equal),
            (Bool(b1), Bool(b2)) => b1.partial_cmp(b2),
            (Char(c1), Char(c2)) => c1.partial_cmp(c2),
            (I8(i1), I8(i2)) => i1.partial_cmp(i2),
            (I16(i1), I16(i2)) => i1.partial_cmp(i2),
            (I32(i1), I32(i2)) => i1.partial_cmp(i2),
            (I64(i1), I64(i2)) => i1.partial_cmp(i2),
            (I128(i1), I128(i2)) => i1.partial_cmp(i2),
            (ISize(i1), ISize(i2)) => i1.partial_cmp(i2),
            (U8(u1), U8(u2)) => u1.partial_cmp(u2),
            (U16(u1), U16(u2)) => u1.partial_cmp(u2),
            (U32(u1), U32(u2)) => u1.partial_cmp(u2),
            (U64(u1), U64(u2)) => u1.partial_cmp(u2),
            (U128(u1), U128(u2)) => u1.partial_cmp(u2),
            (USize(u1), USize(u2)) => u1.partial_cmp(u2),
            (F32(f1), F32(f2)) => f1.partial_cmp(f2),
            (F64(f1), F64(f2)) => f1.partial_cmp(f2),
            (StringLiteral(l0), StringLiteral(r0)) => todo!(),
            (ThawedValue::Owned(l0), ThawedValue::Owned(r0)) => todo!(),
            (Leash(l0), Leash(r0)) => todo!(),
            (OptionBox(l0), OptionBox(r0)) => todo!(),
            (OptionLeash(l0), OptionLeash(r0)) => todo!(),
            (EnumUnit { index: l0, .. }, EnumUnit { index: r0, .. }) => todo!(),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<ThawedValue> for ThawedValue {
    type Output = Self;

    fn add(self, rhs: ThawedValue) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::I8(a), ThawedValue::I8(b)) => ThawedValue::I8(a + b),
            (ThawedValue::I16(a), ThawedValue::I16(b)) => ThawedValue::I16(a + b),
            (ThawedValue::I32(a), ThawedValue::I32(b)) => ThawedValue::I32(a + b),
            (ThawedValue::I64(a), ThawedValue::I64(b)) => ThawedValue::I64(a + b),
            (ThawedValue::I128(a), ThawedValue::I128(b)) => ThawedValue::I128(a + b),
            (ThawedValue::ISize(a), ThawedValue::ISize(b)) => ThawedValue::ISize(a + b),
            (ThawedValue::U8(a), ThawedValue::U8(b)) => ThawedValue::U8(a + b),
            (ThawedValue::U16(a), ThawedValue::U16(b)) => ThawedValue::U16(a + b),
            (ThawedValue::U32(a), ThawedValue::U32(b)) => ThawedValue::U32(a + b),
            (ThawedValue::U64(a), ThawedValue::U64(b)) => ThawedValue::U64(a + b),
            (ThawedValue::U128(a), ThawedValue::U128(b)) => ThawedValue::U128(a + b),
            (ThawedValue::USize(a), ThawedValue::USize(b)) => ThawedValue::USize(a + b),
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a + b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a + b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a + b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a + b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a + b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a + b),
            (ThawedValue::F32(a), ThawedValue::F32(b)) => ThawedValue::F32(a + b),
            (ThawedValue::F64(a), ThawedValue::F64(b)) => ThawedValue::F64(a + b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::AddAssign<ThawedValue> for ThawedValue {
    fn add_assign(&mut self, rhs: ThawedValue) {
        todo!()
    }
}

impl std::ops::BitAnd for ThawedValue {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a & b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a & b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a & b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a & b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a & b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a & b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::BitAndAssign for ThawedValue {
    fn bitand_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::BitOr for ThawedValue {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a | b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a | b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a | b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a | b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a | b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a | b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::BitOrAssign for ThawedValue {
    fn bitor_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::BitXor for ThawedValue {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::BitXorAssign for ThawedValue {
    fn bitxor_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Div for ThawedValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::I8(a), ThawedValue::I8(b)) => ThawedValue::I8(a / b),
            (ThawedValue::I16(a), ThawedValue::I16(b)) => ThawedValue::I16(a / b),
            (ThawedValue::I32(a), ThawedValue::I32(b)) => ThawedValue::I32(a / b),
            (ThawedValue::I64(a), ThawedValue::I64(b)) => ThawedValue::I64(a / b),
            (ThawedValue::I128(a), ThawedValue::I128(b)) => ThawedValue::I128(a / b),
            (ThawedValue::ISize(a), ThawedValue::ISize(b)) => ThawedValue::ISize(a / b),
            (ThawedValue::U8(a), ThawedValue::U8(b)) => ThawedValue::U8(a / b),
            (ThawedValue::U16(a), ThawedValue::U16(b)) => ThawedValue::U16(a / b),
            (ThawedValue::U32(a), ThawedValue::U32(b)) => ThawedValue::U32(a / b),
            (ThawedValue::U64(a), ThawedValue::U64(b)) => ThawedValue::U64(a / b),
            (ThawedValue::U128(a), ThawedValue::U128(b)) => ThawedValue::U128(a / b),
            (ThawedValue::USize(a), ThawedValue::USize(b)) => ThawedValue::USize(a / b),
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a / b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a / b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a / b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a / b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a / b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a / b),
            (ThawedValue::F32(a), ThawedValue::F32(b)) => ThawedValue::F32(a / b),
            (ThawedValue::F64(a), ThawedValue::F64(b)) => ThawedValue::F64(a / b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul for ThawedValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::I8(a), ThawedValue::I8(b)) => ThawedValue::I8(a * b),
            (ThawedValue::I16(a), ThawedValue::I16(b)) => ThawedValue::I16(a * b),
            (ThawedValue::I32(a), ThawedValue::I32(b)) => ThawedValue::I32(a * b),
            (ThawedValue::I64(a), ThawedValue::I64(b)) => ThawedValue::I64(a * b),
            (ThawedValue::I128(a), ThawedValue::I128(b)) => ThawedValue::I128(a * b),
            (ThawedValue::ISize(a), ThawedValue::ISize(b)) => ThawedValue::ISize(a * b),
            (ThawedValue::U8(a), ThawedValue::U8(b)) => ThawedValue::U8(a * b),
            (ThawedValue::U16(a), ThawedValue::U16(b)) => ThawedValue::U16(a * b),
            (ThawedValue::U32(a), ThawedValue::U32(b)) => ThawedValue::U32(a * b),
            (ThawedValue::U64(a), ThawedValue::U64(b)) => ThawedValue::U64(a * b),
            (ThawedValue::U128(a), ThawedValue::U128(b)) => ThawedValue::U128(a * b),
            (ThawedValue::USize(a), ThawedValue::USize(b)) => ThawedValue::USize(a * b),
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a * b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a * b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a * b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a * b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a * b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a * b),
            (ThawedValue::F32(a), ThawedValue::F32(b)) => ThawedValue::F32(a * b),
            (ThawedValue::F64(a), ThawedValue::F64(b)) => ThawedValue::F64(a * b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::MulAssign for ThawedValue {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Neg for ThawedValue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            ThawedValue::Uninit => todo!(),
            ThawedValue::Invalid => todo!(),
            ThawedValue::Moved => todo!(),
            ThawedValue::Unit(_) => todo!(),
            ThawedValue::Bool(_) => todo!(),
            ThawedValue::Char(_) => todo!(),
            ThawedValue::I8(i) => ThawedValue::I8(-i),
            ThawedValue::I16(i) => ThawedValue::I16(-i),
            ThawedValue::I32(i) => ThawedValue::I32(-i),
            ThawedValue::I64(i) => ThawedValue::I64(-i),
            ThawedValue::I128(i) => ThawedValue::I128(-i),
            ThawedValue::ISize(i) => ThawedValue::ISize(-i),
            ThawedValue::U8(_) => todo!(),
            ThawedValue::U16(_) => todo!(),
            ThawedValue::U32(_) => todo!(),
            ThawedValue::U64(_) => todo!(),
            ThawedValue::U128(_) => todo!(),
            ThawedValue::USize(_) => todo!(),
            ThawedValue::R8(_) => todo!(),
            ThawedValue::R16(_) => todo!(),
            ThawedValue::R32(_) => todo!(),
            ThawedValue::R64(_) => todo!(),
            ThawedValue::R128(_) => todo!(),
            ThawedValue::RSize(_) => todo!(),
            ThawedValue::F32(f) => ThawedValue::F32(-f),
            ThawedValue::F64(f) => ThawedValue::F64(-f),
            ThawedValue::StringLiteral(_) => todo!(),
            ThawedValue::Owned(_) => todo!(),
            ThawedValue::Leash(_) => todo!(),
            ThawedValue::Ref(_) => todo!(),
            ThawedValue::Mut(_) => todo!(),
            ThawedValue::OptionBox(_) => todo!(),
            ThawedValue::OptionLeash(_) => todo!(),
            ThawedValue::OptionSizedRef(_) => todo!(),
            ThawedValue::OptionSizedMut(_) => todo!(),
            ThawedValue::EnumUnit { index, presenter } => todo!(),
        }
    }
}

impl std::ops::Not for ThawedValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shl for ThawedValue {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::ShlAssign for ThawedValue {
    fn shl_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Shr for ThawedValue {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::ShrAssign for ThawedValue {
    fn shr_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl std::ops::Sub for ThawedValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ThawedValue::I8(a), ThawedValue::I8(b)) => ThawedValue::I8(a - b),
            (ThawedValue::I16(a), ThawedValue::I16(b)) => ThawedValue::I16(a - b),
            (ThawedValue::I32(a), ThawedValue::I32(b)) => ThawedValue::I32(a - b),
            (ThawedValue::I64(a), ThawedValue::I64(b)) => ThawedValue::I64(a - b),
            (ThawedValue::I128(a), ThawedValue::I128(b)) => ThawedValue::I128(a - b),
            (ThawedValue::ISize(a), ThawedValue::ISize(b)) => ThawedValue::ISize(a - b),
            (ThawedValue::U8(a), ThawedValue::U8(b)) => ThawedValue::U8(a - b),
            (ThawedValue::U16(a), ThawedValue::U16(b)) => ThawedValue::U16(a - b),
            (ThawedValue::U32(a), ThawedValue::U32(b)) => ThawedValue::U32(a - b),
            (ThawedValue::U64(a), ThawedValue::U64(b)) => ThawedValue::U64(a - b),
            (ThawedValue::U128(a), ThawedValue::U128(b)) => ThawedValue::U128(a - b),
            (ThawedValue::USize(a), ThawedValue::USize(b)) => ThawedValue::USize(a - b),
            (ThawedValue::R8(a), ThawedValue::R8(b)) => ThawedValue::R8(a - b),
            (ThawedValue::R16(a), ThawedValue::R16(b)) => ThawedValue::R16(a - b),
            (ThawedValue::R32(a), ThawedValue::R32(b)) => ThawedValue::R32(a - b),
            (ThawedValue::R64(a), ThawedValue::R64(b)) => ThawedValue::R64(a - b),
            (ThawedValue::R128(a), ThawedValue::R128(b)) => ThawedValue::R128(a - b),
            (ThawedValue::RSize(a), ThawedValue::RSize(b)) => ThawedValue::RSize(a - b),
            (ThawedValue::F32(a), ThawedValue::F32(b)) => ThawedValue::F32(a - b),
            (ThawedValue::F64(a), ThawedValue::F64(b)) => ThawedValue::F64(a - b),
            _ => unreachable!(),
        }
    }
}

impl std::ops::SubAssign for ThawedValue {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl From<Infallible> for ThawedValue {
    fn from(value: Infallible) -> Self {
        todo!()
    }
}

impl From<Value> for ThawedValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Unit(()) => ThawedValue::Unit(()),
            Value::Bool(b) => ThawedValue::Bool(b),
            Value::Char(c) => ThawedValue::Char(c),
            Value::I8(i) => ThawedValue::I8(i),
            Value::I16(i) => ThawedValue::I16(i),
            Value::I32(i) => ThawedValue::I32(i),
            Value::I64(i) => ThawedValue::I64(i),
            Value::I128(i) => ThawedValue::I128(i),
            Value::ISize(i) => ThawedValue::ISize(i),
            Value::U8(u) => ThawedValue::U8(u),
            Value::U16(u) => ThawedValue::U16(u),
            Value::U32(u) => ThawedValue::U32(u),
            Value::U64(u) => ThawedValue::U64(u),
            Value::U128(u) => ThawedValue::U128(u),
            Value::USize(u) => ThawedValue::USize(u),
            Value::R8(r) => ThawedValue::R8(r),
            Value::R16(r) => ThawedValue::R16(r),
            Value::R32(r) => ThawedValue::R32(r),
            Value::R64(r) => ThawedValue::R64(r),
            Value::R128(r) => ThawedValue::R128(r),
            Value::RSize(r) => ThawedValue::RSize(r),
            Value::F32(f) => ThawedValue::F32(f),
            Value::F64(f) => ThawedValue::F64(f),
            Value::StringLiteral(id) => ThawedValue::StringLiteral(id),
            Value::Owned(owned_value) => todo!(),
            Value::Leash(_) => todo!(),
            Value::OptionBox(immortal_dyn) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::EnumUnit { index, presenter } => ThawedValue::EnumUnit { index, presenter },
        }
    }
}
