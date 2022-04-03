mod props;

pub use props::*;
use vm::AnyValueDyn;

#[derive(Clone)]
pub struct BuiltinVisualizer {
    pub compiled: for<'eval> fn(&(dyn AnyValueDyn<'eval> + 'eval)) -> VisualProps,
}

impl std::fmt::Debug for BuiltinVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // f.write_str("BuiltinVisualizer")
    }
}

impl PartialEq for BuiltinVisualizer {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for BuiltinVisualizer {}
