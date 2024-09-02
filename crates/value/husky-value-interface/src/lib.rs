#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
pub mod exception;
pub mod ki_control_flow;
pub mod vm_control_flow;

use exception::IsException;
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, EnumUnitValuePresenter, ValuePresentation,
    ValuePresenterCache,
};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use std::convert::Infallible;
use std::sync::Arc;
pub trait IsValue:
    std::fmt::Debug
    + Sized
    + PartialEq
    + PartialOrd
    + std::ops::Add<Self, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::BitAnd<Self, Output = Self>
    + std::ops::BitAndAssign<Self>
    + std::ops::BitOr<Self, Output = Self>
    + std::ops::BitOrAssign<Self>
    + std::ops::BitXor<Self, Output = Self>
    + std::ops::BitXorAssign<Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::MulAssign<Self>
    + std::ops::Neg<Output = Self>
    + std::ops::Not<Output = Self>
    + std::ops::Shl<Self, Output = Self>
    + std::ops::ShlAssign<Self>
    + std::ops::Shr<Self, Output = Self>
    + std::ops::ShrAssign<Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::SubAssign<Self>
    + From<()>
    + Into<()>
    + From<bool>
    + Into<bool>
    + From<u8>
    + Into<u8>
    + From<u16>
    + Into<u16>
    + From<u32>
    + Into<u32>
    + From<u64>
    + Into<u64>
    + From<u128>
    + Into<u128>
    + From<usize>
    + Into<usize>
    + From<i8>
    + Into<i8>
    + From<i16>
    + Into<i16>
    + From<i32>
    + Into<i32>
    + From<i64>
    + Into<i64>
    + From<i128>
    + Into<i128>
    + From<isize>
    + Into<isize>
    + From<f32>
    + Into<f32>
    + From<f64>
    + Into<f64>
    + From<char>
    + Into<char>
    + From<std::convert::Infallible>
    + 'static
{
    type Exception: IsException;

    fn new_uninit() -> Self;
    // the followings are methods that should be implemented.
    // they are commented out because they would probably be done in a way outside of rust trait system
    // fn from_owned<T>(t: T) -> Self;
    // fn into_owned<T>(self) -> T;
    // fn from_ref<'a, T>(t: &'a T) -> Self;
    // fn into_ref<'a, T>(self) -> &'a T;
    // fn from_leash<T>(t: &'static T) -> Self;
    // fn into_leash<T>(self) -> &'static T;
    // fn from_mut<'a, T>(t: &'a mut T) -> Self;
    // fn into_mut<'a, T>(self) -> &'a mut T;
    // fn from_option_ref<'a, T>(t: Option<&'a T>) -> Self;
    // fn into_option_ref<'a, T>(self) -> Option<&'a T>;
    // fn from_option_mut<'a, T>(t: Option<&'a mut T>) -> Self;
    // fn into_option_mut<'a, T>(self) -> Option<&'a mut T>;
    /// `Arc<str>` should be replaced with something better
    fn from_str_literal(str_value: Arc<str>) -> Self;
    fn from_enum_index(index: usize, presenter: EnumUnitValuePresenter) -> Self;
    fn share(&'static self) -> Self;
    fn to_bool(self) -> bool;
    fn to_usize(self) -> usize;
    fn r#move(&mut self) -> Self;
    /// should unreachable if not an option
    fn is_none(self) -> bool;
    /// should unreachable if not an option
    fn is_some(self) -> bool;
    fn index(self, index: usize) -> Result<Self, Self::Exception>;
    fn unwrap(self) -> Result<Self, Self::Exception>;
    fn present(
        &self,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> ValuePresentation;

    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual;

    ///# vm
    type FrozenValue;
    fn freeze(&self) -> Self::FrozenValue;
}
