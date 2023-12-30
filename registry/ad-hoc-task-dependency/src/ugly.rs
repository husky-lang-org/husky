pub use crate::{FromValue as __FromValue, IntoValue as __IntoValue, Value as __Value};
pub use husky_linkage_impl::standard::ugly::*;
pub use husky_ml_task_interface::ugly::*;
pub use husky_standard_value::ugly::*;
pub use husky_task_interface::ugly::*;

use husky_linkage_impl::standard::LinkageImpl;
use husky_task_interface::DevEvalContext;

pub type __DevEvalContext = DevEvalContext<__LinkageImpl>;
pub type __LinkageImpl = LinkageImpl<__Pedestal>;
pub type __ClosedPoint = __InputId;
// ad hoc
pub type __ValueResult = Result<__Value, ()>;