pub mod storage;

use crate::*;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_math_letter::letter::LxMathLetter;
use visored_entity_path::module::VdModulePath;
use visored_sem_expr::symbol::local_defn::{
    storage::VdSemSymbolLocalDefnStorage, VdSemSymbolLocalDefnBody, VdSemSymbolLocalDefnData,
    VdSemSymbolLocalDefnHead, VdSemSymbolLocalDefnIdx,
};

pub struct VdMirSymbolLocalDefnData {
    head: VdMirSymbolLocalDefnHead,
    body: VdMirSymbolLocalDefnBody,
    module_path: VdModulePath,
}

pub enum VdMirSymbolLocalDefnHead {
    Letter(LxMathLetter),
}

pub enum VdMirSymbolLocalDefnBody {
    Assigned,
    Placeholder,
}

pub type VdMirSymbolLocalDefnArena = Arena<VdMirSymbolLocalDefnData>;
pub type VdMirSymbolLocalDefnArenaRef<'a> = ArenaRef<'a, VdMirSymbolLocalDefnData>;
pub type VdMirSymbolLocalDefnIdx = ArenaIdx<VdMirSymbolLocalDefnData>;
pub type VdMirSymbolLocalDefnIdxRange = ArenaIdxRange<VdMirSymbolLocalDefnData>;
pub type VdMirSymbolLocalDefnMap<T> = ArenaMap<VdMirSymbolLocalDefnData, T>;
pub type VdMirSymbolLocalDefnOrderedMap<T> = ArenaOrderedMap<VdMirSymbolLocalDefnData, T>;

impl VdMirSymbolLocalDefnData {
    pub fn head(&self) -> &VdMirSymbolLocalDefnHead {
        &self.head
    }

    pub fn body(&self) -> &VdMirSymbolLocalDefnBody {
        &self.body
    }

    pub fn module_path(&self) -> VdModulePath {
        self.module_path
    }
}

impl<'a> VdMirExprRegionBuilder<'a> {
    pub(crate) fn build_symbol_local_defns(
        &mut self,
        sem_symbol_local_defn_storage: &VdSemSymbolLocalDefnStorage,
    ) {
        let mut defns = vec![];
        for defn in sem_symbol_local_defn_storage.defn_arena() {
            defns.push(self.build_symbol_local_defn(defn));
        }
        self.alloc_symbol_local_defns(defns);
    }

    fn build_symbol_local_defn(
        &mut self,
        defn: &VdSemSymbolLocalDefnData,
    ) -> VdMirSymbolLocalDefnData {
        VdMirSymbolLocalDefnData {
            head: defn.head().to_vd_mir(self),
            body: defn.body().to_vd_mir(self),
            module_path: defn.module_path(),
        }
    }
}

impl<'db> ToVdMir<VdMirSymbolLocalDefnIdx, VdMirExprRegionBuilder<'db>>
    for VdSemSymbolLocalDefnIdx
{
    fn to_vd_mir(self, builder: &mut VdMirExprRegionBuilder<'db>) -> VdMirSymbolLocalDefnIdx {
        // INVARIANCE: the index is always the same as the syn index
        unsafe { VdMirSymbolLocalDefnIdx::new_ext(self.index()) }
    }
}

impl<'db> ToVdMir<VdMirSymbolLocalDefnHead, VdMirExprRegionBuilder<'db>>
    for &VdSemSymbolLocalDefnHead
{
    fn to_vd_mir(self, builder: &mut VdMirExprRegionBuilder<'db>) -> VdMirSymbolLocalDefnHead {
        match *self {
            VdSemSymbolLocalDefnHead::Letter(_, letter) => VdMirSymbolLocalDefnHead::Letter(letter),
        }
    }
}

impl<'db> ToVdMir<VdMirSymbolLocalDefnBody, VdMirExprRegionBuilder<'db>>
    for &VdSemSymbolLocalDefnBody
{
    fn to_vd_mir(self, builder: &mut VdMirExprRegionBuilder<'db>) -> VdMirSymbolLocalDefnBody {
        match self {
            VdSemSymbolLocalDefnBody::Assigned => VdMirSymbolLocalDefnBody::Assigned,
            VdSemSymbolLocalDefnBody::Placeholder => VdMirSymbolLocalDefnBody::Placeholder,
        }
    }
}
