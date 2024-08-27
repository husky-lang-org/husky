pub mod accompany;
pub(crate) mod action;
pub mod bundle;
mod entry;

pub use self::action::TraceSynchrotronAction;
pub use self::entry::TraceSynchrotronEntry;

use self::action::TraceSynchrotronActionsDiff;
use self::bundle::TraceIdBundle;
use crate::synchrotron::accompany::AccompanyingTraceIdsExceptFollowed;
use crate::{view::TraceViewData, *};
use figure::{FigureKey, TraceFigureKey};
use husky_item_path_interface::ItemPathIdInterface;
use husky_value_protocol::presentation::synchrotron::{
    ValuePresentationSynchrotron, ValuePresentationSynchrotronStatus,
};
use husky_visual_protocol::synchrotron::{action::VisualSynchrotronStatus, VisualSynchrotron};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::path::{Path, PathBuf};

/// contains information about traces that are synced across server and client
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotron<TraceProtocol: IsTraceProtocol> {
    caryatid: TraceProtocol::Caryatid,
    accompanyings: AccompanyingTraceIds,
    trace_id_bundles: Vec<TraceIdBundle>,
    followed_trace_id: Option<TraceId>,
    #[serde_as(as = "Vec<(_, _)>")]
    entries: FxHashMap<TraceId, TraceSynchrotronEntry<TraceProtocol>>,
    actions: Vec<TraceSynchrotronAction<TraceProtocol>>,
    #[serde_as(as = "Vec<(_, _)>")]
    figures: FxHashMap<TraceFigureKey<TraceProtocol>, TraceProtocol::Figure>,
    // child synchrotrons
    value_presentation_synchrotron: ValuePresentationSynchrotron,
    visual_synchrotron: VisualSynchrotron,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronStatus {
    actions_len: usize,
    value_presentation_synchrotron_status: ValuePresentationSynchrotronStatus,
    visual_synchrotron_status: VisualSynchrotronStatus,
}

/// # methods
impl<TraceProtocol: IsTraceProtocol> TraceSynchrotron<TraceProtocol> {
    pub(crate) fn new<Trace: IsTrace>(
        trace_bundles: &[TraceBundle<Trace>],
        trace_var_deps_and_view_data: impl Fn(
            Trace,
        )
            -> (SmallVec<[ItemPathIdInterface; 2]>, TraceViewData),
    ) -> Self {
        let mut entries: FxHashMap<TraceId, TraceSynchrotronEntry<TraceProtocol>> =
            Default::default();
        let mut trace_id_bundles: Vec<TraceIdBundle> = vec![];
        for trace_bundle in trace_bundles {
            let mut root_trace_ids: Vec<TraceId> = vec![];
            for &root_trace in trace_bundle.root_traces() {
                let root_trace_id = root_trace.into();
                root_trace_ids.push(root_trace_id);
                let (var_deps, view_data) = trace_var_deps_and_view_data(root_trace);
                assert!(entries
                    .insert(
                        root_trace_id,
                        TraceSynchrotronEntry::new(var_deps, view_data)
                    )
                    .is_none())
            }
            trace_id_bundles.push(TraceIdBundle::new(
                trace_bundle.crate_root_module_file_abs_path().to_owned(),
                root_trace_ids,
            ))
        }
        Self {
            caryatid: Default::default(),
            trace_id_bundles,
            entries,
            actions: vec![],
            value_presentation_synchrotron: Default::default(),
            visual_synchrotron: Default::default(),
            followed_trace_id: None,
            accompanyings: Default::default(),
            figures: Default::default(),
        }
    }

    pub fn trace_id_bundles(&self) -> &[TraceIdBundle] {
        &self.trace_id_bundles
    }

    pub fn visual_synchrotron(&self) -> &VisualSynchrotron {
        &self.visual_synchrotron
    }

    #[track_caller]
    pub fn figure(&self) -> &TraceProtocol::Figure {
        // #[cfg(test)]
        // {
        //     if !self.figures.contains_key(&self.current) {
        //         husky_io_utils::diff_write(
        //             "trace_synchrotron_failure_actions.log",
        //             format!("{:#?}", self.actions),
        //             true,
        //         );
        //         panic!("trace synchrotron failed to maintain valid state")
        //     }
        // }
        &self.figures[&self.figure_key()]
    }

    pub fn figure_key(&self) -> TraceFigureKey<TraceProtocol> {
        todo!()
    }

    pub(crate) fn status(&self) -> TraceSynchrotronStatus {
        TraceSynchrotronStatus {
            actions_len: self.actions.len(),
            value_presentation_synchrotron_status: self.value_presentation_synchrotron.status(),
            visual_synchrotron_status: self.visual_synchrotron.status(),
        }
    }

    pub fn accompanied(&self, trace_id: TraceId) -> bool {
        self.accompanyings.contains(&trace_id)
    }

    pub(crate) fn actions_diff(
        &self,
        previous_trace_synchrotron_status: TraceSynchrotronStatus,
    ) -> TraceSynchrotronActionsDiff<TraceProtocol> {
        assert!(previous_trace_synchrotron_status.actions_len < self.actions.len());
        let actions = self.actions[previous_trace_synchrotron_status.actions_len..]
            .iter()
            .map(|action| action.clone())
            .collect();
        let value_presentation_actions_diff = self
            .value_presentation_synchrotron
            .actions_diff(previous_trace_synchrotron_status.value_presentation_synchrotron_status);
        let visual_actions_diff = self
            .visual_synchrotron
            .actions_diff(previous_trace_synchrotron_status.visual_synchrotron_status);
        TraceSynchrotronActionsDiff::new(
            actions,
            value_presentation_actions_diff,
            visual_actions_diff,
        )
    }

    pub(crate) fn is_trace_cached(&self, trace_id: TraceId) -> bool {
        self.entries.contains_key(&trace_id)
    }

    pub(crate) fn trace_listing(&self) -> Vec<TraceId> {
        let mut trace_listings: Vec<TraceId> = vec![];
        for trace_bundle in self.trace_id_bundles() {
            for &root_trace_id in trace_bundle.root_trace_ids() {
                self.trace_listing_aux(root_trace_id, &mut trace_listings)
            }
        }
        trace_listings
    }

    fn trace_listing_aux(&self, trace_id: TraceId, trace_listings: &mut Vec<TraceId>) {
        trace_listings.push(trace_id);
        let entry = &self[trace_id];
        for &assoc_trace_id in entry.assoc_trace_ids() {
            self.trace_listing_aux(assoc_trace_id, trace_listings)
        }
        if entry.expanded() {
            for &subtrace_id in entry.subtrace_ids().unwrap() {
                self.trace_listing_aux(subtrace_id, trace_listings)
            }
        }
    }

    pub fn caryatid(&self) -> &TraceProtocol::Caryatid {
        &self.caryatid
    }

    pub(crate) fn value_presentation_synchrotron_mut(
        &mut self,
    ) -> &mut ValuePresentationSynchrotron {
        &mut self.value_presentation_synchrotron
    }

    pub fn followed(&self) -> Option<TraceId> {
        self.followed_trace_id
    }

    pub fn accompanying_trace_ids(&self) -> &AccompanyingTraceIds {
        &self.accompanyings
    }

    pub fn accompanyings_except_followed(
        &self,
        followed: Option<TraceId>,
    ) -> AccompanyingTraceIdsExceptFollowed {
        AccompanyingTraceIdsExceptFollowed::new(followed, self.accompanyings.clone())
    }

    pub fn has_figure(&self, figure_key: &TraceFigureKey<TraceProtocol>) -> bool {
        self.figures.contains_key(&figure_key)
    }

    pub(crate) fn cache_figure(
        &mut self,
        key: TraceFigureKey<TraceProtocol>,
        figure: TraceProtocol::Figure,
    ) {
        assert!(self.figures.insert(key, figure).is_none())
    }

    pub(crate) fn visual_synchrotron_mut(&mut self) -> &mut VisualSynchrotron {
        &mut self.visual_synchrotron
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::Index<TraceId> for TraceSynchrotron<TraceProtocol> {
    type Output = TraceSynchrotronEntry<TraceProtocol>;

    fn index(&self, id: TraceId) -> &Self::Output {
        &self.entries[&id]
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::IndexMut<TraceId>
    for TraceSynchrotron<TraceProtocol>
{
    #[track_caller]
    fn index_mut(&mut self, id: TraceId) -> &mut Self::Output {
        self.entries.get_mut(&id).unwrap()
    }
}
