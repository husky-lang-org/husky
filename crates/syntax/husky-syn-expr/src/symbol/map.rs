use super::*;
use idx_arena::ArenaIdx;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb, jar = SynExprJar)]
pub struct SymbolMap<V> {
    inherited_syn_symbol_map: InheritedSynSymbolMap<V>,
    current_syn_symbol_map: CurrentSynSymbolMap<V>,
}

impl<V> SymbolMap<V> {
    pub fn new(parent: Option<&Self>, region: &SynSymbolRegionData) -> Self
    where
        V: Clone,
    {
        let inherited_syn_symbol_arena = region.inherited_syn_symbol_arena();
        let current_syn_symbol_arena = region.current_syn_symbol_arena();
        Self {
            inherited_syn_symbol_map: match parent {
                Some(parent) => {
                    let mut inherited_syn_symbol_map = parent
                        .inherited_syn_symbol_map
                        .clone_for_extended(inherited_syn_symbol_arena);
                    let base = parent.inherited_syn_symbol_map.len();
                    for (parent_idx, v) in parent.current_syn_symbol_map.key_value_iter() {
                        unsafe {
                            let idx = ArenaIdx::from_raw(base + parent_idx.index());
                            inherited_syn_symbol_map.insert_new(idx, v.clone())
                        }
                    }
                    inherited_syn_symbol_map
                }
                None => InheritedSynSymbolMap::new(inherited_syn_symbol_arena),
            },
            current_syn_symbol_map: CurrentSynSymbolMap::new(current_syn_symbol_arena),
        }
    }

    pub fn insert_new(&mut self, idx: CurrentSynSymbolIdx, v: V) {
        self.current_syn_symbol_map.insert_new(idx, v)
    }

    pub fn inherited_syn_symbol_map(&self) -> &InheritedSynSymbolMap<V> {
        &self.inherited_syn_symbol_map
    }

    pub fn current_syn_symbol_map(&self) -> &CurrentSynSymbolMap<V> {
        &self.current_syn_symbol_map
    }

    pub fn current_syn_symbols(&self) -> &CurrentSynSymbolMap<V> {
        &self.current_syn_symbol_map
    }

    pub fn inherited_syn_symbol_key_values(
        &self,
    ) -> impl Iterator<Item = (InheritedSynSymbolIdx, &V)> {
        self.inherited_syn_symbol_map.key_value_iter()
    }

    pub fn current_syn_symbol_key_values(&self) -> impl Iterator<Item = (CurrentSynSymbolIdx, &V)> {
        self.current_syn_symbol_map.key_value_iter()
    }
}

impl<V> std::ops::Index<InheritedSynSymbolIdx> for SymbolMap<V> {
    type Output = V;

    fn index(&self, index: InheritedSynSymbolIdx) -> &Self::Output {
        &self.inherited_syn_symbol_map[index]
    }
}

impl<V> std::ops::Index<CurrentSynSymbolIdx> for SymbolMap<V> {
    type Output = V;

    fn index(&self, index: CurrentSynSymbolIdx) -> &Self::Output {
        &self.current_syn_symbol_map[index]
    }
}
