use crate::clause::VdSynClauseIdxRange;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdSynSentenceData {
    Clauses(VdSynClauseIdxRange),
}

pub type VdSynSentenceArena = Arena<VdSynSentenceData>;
pub type VdSynSentenceArenaRef<'a> = ArenaRef<'a, VdSynSentenceData>;
pub type VdSynSentenceIdx = ArenaIdx<VdSynSentenceData>;
pub type VdSynSentenceIdxRange = ArenaIdxRange<VdSynSentenceData>;
