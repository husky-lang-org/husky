use idx_arena::ordered_map::ArenaOrderedMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub enum PatternSynSymbol {
    Atom(PatternSynExprIdx),
}

impl PatternSynSymbol {
    pub(super) fn pattern_symbol_modifier(
        &self,
        pattern_expr_arena: &PatternSynExprArena,
    ) -> SymbolModifier {
        match self {
            PatternSynSymbol::Atom(expr_idx) => match pattern_expr_arena[*expr_idx] {
                PatternSynExpr::Ident {
                    symbol_modifier_keyword_group,
                    ident_token,
                } => SymbolModifier::new(symbol_modifier_keyword_group),
                _ => unreachable!(),
            },
        }
    }
}

pub type PatternSymbolArena = Arena<PatternSynSymbol>;
pub type PatternSymbolIdx = ArenaIdx<PatternSynSymbol>;
pub type PatternSymbolMap<V> = ArenaMap<PatternSynSymbol, V>;
pub type PatternSymbolOrderedMap<V> = ArenaOrderedMap<PatternSynSymbol, V>;
