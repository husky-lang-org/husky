//! As we expect Lean in our projects to be autogenerated by visored, we group defns into arena for convenience
pub mod def;
pub mod variable;

use self::{def::*, variable::*};
use crate::*;
use expr::LnMirExprIdx;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use lean_coword::ident::LnIdent;
use lean_entity_path::namespace::LnNamespace;

pub enum LnItemDefnData {
    Variable {
        ident: LnIdent,
        ty: LnMirExprIdx,
    },
    Def {
        ident: LnIdent,
        parameters: Vec<LnDefParameter>,
        // 'None' means unit type
        ty: Option<LnMirExprIdx>,
        body: LnMirDefBody,
    },
    Group {
        defns: LnItemDefnIdxRange,
        meta: LnMirItemDefnGroupMeta,
    },
}

pub enum LnItemDefnChild {
    Expr(LnMirExprIdx),
    Defn(LnItemDefnIdx),
    DefBody(LnMirDefBody),
}

impl LnItemDefnData {
    pub(crate) fn children(&self) -> Vec<LnItemDefnChild> {
        match *self {
            LnItemDefnData::Variable { .. } => vec![],
            LnItemDefnData::Group { defns, .. } => {
                defns.into_iter().map(LnItemDefnChild::Defn).collect()
            }
            LnItemDefnData::Def {
                ident,
                ref parameters,
                ty,
                body,
            } => {
                let mut children: Vec<_> = parameters
                    .into_iter()
                    .map(|param| LnItemDefnChild::Expr(param.ty))
                    .collect();
                if let Some(ty) = ty {
                    children.push(LnItemDefnChild::Expr(ty));
                }
                children.push(LnItemDefnChild::DefBody(body));
                children
            }
        }
    }
}

pub enum LnMirItemDefnGroupMeta {
    Paragraph,
    Sentence,
    Division(Option<LnNamespace>),
    Environment(LnNamespace),
}

pub type LnItemDefnArena = Arena<LnItemDefnData>;
pub type LnItemDefnMap<T> = ArenaMap<LnItemDefnData, T>;
pub type LnItemDefnOrderedMap<T> = ArenaOrderedMap<LnItemDefnData, T>;
pub type LnItemDefnArenaRef<'a> = ArenaRef<'a, LnItemDefnData>;
pub type LnItemDefnIdx = ArenaIdx<LnItemDefnData>;
pub type LnItemDefnIdxRange = ArenaIdxRange<LnItemDefnData>;

impl std::fmt::Display for LnMirItemDefnGroupMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LnMirItemDefnGroupMeta::Paragraph => write!(f, "paragraph"),
            LnMirItemDefnGroupMeta::Sentence => write!(f, "sentence"),
            LnMirItemDefnGroupMeta::Division(_) => write!(f, "division"),
            LnMirItemDefnGroupMeta::Environment(_) => write!(f, "environment"),
        }
    }
}

pub enum LnItemDefnComment {
    Void,
    Lines(Vec<String>),
    Qed,
}

pub type LnItemDefnCommentMap = LnItemDefnOrderedMap<LnItemDefnComment>;

impl LnItemDefnComment {
    pub fn from_latex_source(input: &str) -> Self {
        let lines = input
            .lines()
            .into_iter()
            .map(|line| line.to_string())
            .collect();
        Self::Lines(lines)
    }
}
