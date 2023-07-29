//!
//! Given struct A { x: &mut T, ys: Vec<i32>, .. } Regular Value Type
//! - struct __StaticA { x: *mut T, ys: Vec<i32>, .. } Regular Value is its Static version,
//! which doesn't have extra heap allocation
//! - struct __SnapshotA { x: Box<T>, ys: Vec<i32>, .. } is its Snapshot version,
//! which does have extra heap allocation
//! - struct __StandA { x: Box<T>, ys: ManuallyDrop<Vec<i32>>, .. } is its Stand version with Drop customized,
//! which does have extra heap allocation
//!
//! All four types have the same size
//!
//! ```rust
//! struct A {
//!     x: &mut T,
//!     ys: Vec<i32>,
//! }
//!
//! impl __Regular for A {
//!     type __StaticSelf = __StaticA;
//! }
//!
//! struct __StaticA {
//!     x: *mut T,
//!     ys: Vec<i32>,
//! }
//!
//! impl __RegularStatic for __StaticA {
//!     type __SnapshotSelf = __SnapshotA;
//! }
//! ```

#[cfg(feature = "constant")]
mod constant;
mod regular;
#[cfg(feature = "vm_support")]
mod snapshot;
mod snapshot_dyn;
#[cfg(feature = "vm_support")]
mod stand;
mod stand_dyn;
mod static_info;
mod value;
mod value_dyn;

use std::num::NonZeroU32;

#[cfg(feature = "constant")]
pub use self::constant::*;
pub use self::regular::*;
#[cfg(feature = "vm_support")]
pub use self::snapshot::*;
pub use self::snapshot_dyn::*;
#[cfg(feature = "vm_support")]
pub use self::stand::*;
pub use self::stand_dyn::*;
pub use self::static_info::*;
pub use self::value::*;

#[cfg(feature = "constant")]
use husky_term_prelude::*;
use std::sync::Arc;
