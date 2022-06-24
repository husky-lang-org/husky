use dev_utils::{DevSource, StaticDevSource};

use crate::*;

/// RoutineLinkage
#[derive(Clone, Copy)]
pub struct RoutineLinkage {
    pub call: for<'temp, 'eval> fn(
        &mut [TempValue<'temp, 'eval>],
    ) -> VMRuntimeResult<TempValue<'temp, 'eval>>,
    pub nargs: u8,
    pub dev_src: &'static StaticDevSource,
}

impl std::fmt::Debug for RoutineLinkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("RoutineFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for RoutineLinkage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for RoutineLinkage {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for RoutineLinkage {}

/// ModelLinkage
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModelLinkage {
    pub train: (),
    pub eval: (),
}

#[macro_export]
macro_rules! linkage {
    ($fp: expr, $nargs: expr) => {{
        RoutineLinkage {
            call: $fp,
            nargs: $nargs,
            dev_src: &dev_utils::static_dev_src!(),
        }
    }};
}
