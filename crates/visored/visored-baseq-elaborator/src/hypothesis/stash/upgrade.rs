use super::*;
use crate::hypothesis::{
    stack::VdBsqHypothesisStackRecord, VdBsqHypothesisEntry, VdBsqHypothesisIdx,
};
use rustc_hash::FxHashMap;
use std::marker::PhantomData;

pub trait IsVdBsqHypothesisUpdateStashScheme: IsVdBsqHypothesisStashScheme {}

pub struct VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateStashScheme,
{
    map: FxHashMap<Scheme::Key<'sess>, VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>>,
}

pub struct VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateStashScheme,
{
    values: Vec<(VdBsqHypothesisStackRecord<'sess>, Scheme::Value<'sess>)>,
}

impl<'sess, Scheme> Default for VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateStashScheme,
{
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateStashScheme,
{
    fn clear_inactive_values(
        &mut self,
        stack_idx: usize,
        is_active: impl Fn(VdBsqHypothesisStackRecord<'sess>) -> bool,
    ) {
        while let Some(&(stack_record, _)) = self.values.last()
            && !is_active(stack_record)
        {
            self.values.pop();
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateStashScheme,
{
    pub fn cache(
        &mut self,
        hypothesis_record: VdBsqHypothesisStackRecord<'sess>,
        hypothesis_data: &VdBsqHypothesisEntry<'sess>,
    ) {
        todo!()
    }
}
