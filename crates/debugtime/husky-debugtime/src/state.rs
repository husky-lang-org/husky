mod hot_reload;
mod update;

pub use hot_reload::*;
use proj_like::{ProjAtom, ProjMap, ProjVec};
pub use update::*;

use crate::*;

#[derive(Default)]
pub struct DebugtimeState {
    pub(crate) restriction: ProjAtom<Restriction>,
    pub(crate) pins: VecSet<TraceId>,
    pub(crate) opt_active_trace_id: ProjAtom<Option<TraceId>>,
    pub(crate) trace_nodes: ProjVec<Option<TraceNode>>,
    pub(crate) figure_canvases: ProjMap<FigureCanvasKey, FigureCanvasData>,
    pub(crate) figure_controls: ProjMap<FigureControlKey, FigureControlData>,
    pub(crate) trace_stalks: ProjMap<TraceStalkKey, TraceStalk>,
    pub(crate) trace_statss: ProjMap<TraceStatsKey, Option<TraceStats>>,
    root_traces: Vec<TraceId>,
    pub(crate) subtrace_ids_map: HashMap<SubtracesKey, Vec<TraceId>>,
}

impl DebugtimeState {
    pub(crate) fn root_traces(&self) -> &[TraceId] {
        &self.root_traces
    }

    pub(crate) fn set_root_traces(&mut self, root_traces: Vec<TraceId>) {
        self.root_traces = root_traces
    }
}
