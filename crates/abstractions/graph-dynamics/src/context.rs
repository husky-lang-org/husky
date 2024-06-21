use propagate::{PropagationResult, PropagationResultRef};

use crate::{
    cycle_group::{CycleGroup, CycleGroupMap},
    final_values::calc_cycle_group_final_values,
    full_deps_cropped::calc_full_deps_cropped,
};

pub trait IsGraphScheme {
    type Node: Eq + Ord + Copy + 'static;
    const CYCLE_GROUP_N: usize;
    type CycleGroupItd: Copy;
}

pub trait IsGraphDynamicsScheme: IsGraphScheme + 'static {
    type Value: Eq + std::fmt::Debug;
    const MAX_ITERATION: usize;
}

pub trait IsGraphDynamicsContext<'db>: Copy {
    type Scheme: IsGraphDynamicsScheme;
    /// crop deps that definitely are not going to form a cycle
    fn deps_cropped(self, node: Node<Self::Scheme>)
        -> impl IntoIterator<Item = Node<Self::Scheme>>;
    /// cached version
    fn full_deps_cropped(self, node: Node<Self::Scheme>) -> &'db [Node<Self::Scheme>];

    /// cached version
    fn cycle_group_itd(self, node: Node<Self::Scheme>) -> CycleGroupItd<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:;
    fn initial_value(self, node: Node<Self::Scheme>) -> Value<Self::Scheme>;
    fn updated_value<'a>(
        self,
        node: Node<Self::Scheme>,
        query: impl Fn(Node<Self::Scheme>) -> &'a Value<Self::Scheme>,
    ) -> Value<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:;
    /// final
    fn calc_cycle_group_final_values(
        self,
        cycle_group: &'db CycleGroup<Self::Scheme>,
    ) -> PropagationResult<CycleGroupMap<Self::Scheme>>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:,
    {
        calc_cycle_group_final_values(self, cycle_group)
    }
    /// cached version
    fn cycle_group_values(
        self,
        cycle_group_itd: CycleGroupItd<Self::Scheme>,
    ) -> PropagationResultRef<'db, &'db CycleGroupMap<Self::Scheme>>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:;
    /// go through interned cycle group
    fn value(self, node: Node<Self::Scheme>) -> PropagationResultRef<'db, &'db Value<Self::Scheme>>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:,
    {
        Ok(&self.cycle_group_values(self.cycle_group_itd(node))?[node])
    }
}

type Node<S> = <S as IsGraphScheme>::Node;
type CycleGroupItd<S> = <S as IsGraphScheme>::CycleGroupItd;
type Value<S> = <S as IsGraphDynamicsScheme>::Value;
