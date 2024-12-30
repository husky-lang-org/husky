pub mod elaboration;

use elaboration::VdMirTacticElaboration;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use once_place::OncePlace;

pub enum VdMirTacticData {
    Obvious,
}

pub struct VdMirTacticEntry {
    data: VdMirTacticData,
    elaboration: OncePlace<VdMirTacticElaboration>,
}

impl VdMirTacticEntry {
    pub fn new(data: VdMirTacticData) -> Self {
        Self {
            data,
            elaboration: OncePlace::default(),
        }
    }
}

pub type VdMirTacticIdx = ArenaIdx<VdMirTacticEntry>;
pub type VdMirTacticIdxRange = ArenaIdxRange<VdMirTacticEntry>;
pub type VdMirTacticArena = Arena<VdMirTacticEntry>;
