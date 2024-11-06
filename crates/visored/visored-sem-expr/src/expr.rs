pub mod attach;
pub mod binary;
pub mod literal;
pub mod notation;
pub mod prefix;
pub mod suffix;
#[cfg(test)]
pub mod tests;
pub mod uniadic_array;
pub mod uniadic_chain;
pub mod variadic_array;
pub mod variadic_chain;

use self::{attach::AttachDispatch, binary::VdSemBinaryDispatch};
use crate::*;
use either::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_math_letter::LxMathLetter;
use latex_prelude::script::LxScriptKind;
use latex_token::idx::{LxMathTokenIdx, LxTokenIdx, LxTokenIdxRange};
use visored_opr::{
    delimiter::{
        VdBaseLeftDelimiter, VdBaseRightDelimiter, VdCompositeLeftDelimiter,
        VdCompositeRightDelimiter,
    },
    opr::{
        binary::{VdBaseBinaryOpr, VdCompositeBinaryOpr},
        prefix::{VdBasePrefixOpr, VdCompositePrefixOpr},
        suffix::{VdBaseSuffixOpr, VdCompositeSuffixOpr},
        VdBaseOpr,
    },
    separator::{VdBaseSeparator, VdCompositeSeparator},
};
use visored_syn_expr::expr::{VdSynExprData, VdSynSeparator};
use visored_zfc_ty::term::literal::VdZfcLiteral;

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSemExprData {
    Literal {
        token_idx_range: LxTokenIdxRange,
        literal: VdZfcLiteral,
    },
    // TODO: split into namespace and variable, using dispatch??
    Letter {
        token_idx_range: LxTokenIdxRange,
        letter: LxMathLetter,
    },
    BaseOpr {
        opr: VdBaseOpr,
    },
    Binary {
        lopd: VdSemExprIdx,
        opr: VdSemBinaryOpr,
        ropd: VdSemExprIdx,
        dispatch: VdSemBinaryDispatch,
    },
    Prefix {
        opr: VdSemPrefixOpr,
        opd: VdSemExprIdx,
        dispatch: (),
    },
    Suffix {
        opd: VdSemExprIdx,
        opr: VdSemExprIdx,
        dispatch: (),
    },
    Attach {
        base: VdSemExprIdx,
        // INVARIANCE: at least one of these are some
        scripts: Vec<(LxScriptKind, VdSemExprIdx)>,
        dispatch: AttachDispatch,
    },
    SeparatedList {
        fragments: Vec<Either<VdSemExprIdx, VdSemSeparator>>,
    },
    // TODO: maybe these two are just separated lists?
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
    LxDelimited {
        left_delimiter_token_idx: LxMathTokenIdx,
        right_delimiter_token_idx: LxMathTokenIdx,
    },
    Delimited {
        left_delimiter: VdSemLeftDelimiter,
        right_delimiter: VdSemRightDelimiter,
    },
    Fraction {
        command_token_idx: LxMathTokenIdx,
        denominator_rcurl_token_idx: LxMathTokenIdx,
    },
    Sqrt {
        command_token_idx: LxMathTokenIdx,
        radicand_rcurl_token_idx: LxMathTokenIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemPrefixOpr {
    Base(LxTokenIdxRange, VdBasePrefixOpr),
    Composite(VdSemExprIdx, VdCompositePrefixOpr),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemSuffixOpr {
    Base(LxTokenIdxRange, VdBaseSuffixOpr),
    Composite(VdSemExprIdx, VdCompositeSuffixOpr),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemBinaryOpr {
    Base(LxTokenIdxRange, VdBaseBinaryOpr),
    Composite(VdSemExprIdx, VdCompositeBinaryOpr),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemLeftDelimiter {
    Base(LxTokenIdxRange, VdBaseLeftDelimiter),
    Composite(VdSemExprIdx, VdCompositeLeftDelimiter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemRightDelimiter {
    Base(LxTokenIdxRange, VdBaseRightDelimiter),
    Composite(VdSemExprIdx, VdCompositeRightDelimiter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemSeparator {
    Base(LxTokenIdxRange, VdBaseSeparator),
    Composite(VdSemExprIdx, VdCompositeSeparator),
}

pub type VdSemExprIdx = ArenaIdx<VdSemExprData>;
pub type VdSemExprIdxRange = ArenaIdxRange<VdSemExprData>;
pub type VdSemExprArena = Arena<VdSemExprData>;
pub type VdSemExprArenaRef<'a> = ArenaRef<'a, VdSemExprData>;
pub type VdSemExprMap<T> = ArenaMap<VdSemExprData, T>;

impl ToVdSem<VdSemExprIdx> for VdSynExprIdx {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemExprIdx {
        if let Some(&idx) = builder.syn_to_sem_expr_map().get(self) {
            return idx;
        }
        let data = builder.build_expr(self);
        builder.alloc_expr(self, data)
    }
}

impl<'db> VdSemExprBuilder<'db> {
    fn build_expr(&mut self, syn_expr: VdSynExprIdx) -> VdSemExprData {
        match self.syn_expr_arena()[syn_expr] {
            VdSynExprData::Literal {
                token_idx_range,
                literal,
            } => VdSemExprData::Literal {
                token_idx_range,
                literal,
            },
            VdSynExprData::Notation => todo!(),
            VdSynExprData::Letter {
                token_idx_range,
                letter,
            } => todo!(),
            VdSynExprData::BaseOpr { opr } => todo!(),
            VdSynExprData::Binary { lopd, opr, ropd } => todo!(),
            VdSynExprData::Prefix { opr, opd } => todo!(),
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::SeparatedList {
                separator,
                ref fragments,
            } => VdSemExprData::SeparatedList {
                fragments: fragments
                    .iter()
                    .copied()
                    .map(|fragment| fragment.to_vd_sem(self))
                    .collect(),
            },
            VdSynExprData::LxDelimited {
                left_delimiter_token_idx,
                left_delimiter,
                item,
                right_delimiter_token_idx,
                right_delimiter,
            } => todo!(),
            VdSynExprData::Delimited {
                left_delimiter,
                item,
                right_delimiter,
            } => todo!(),
            VdSynExprData::Attach { base, ref scripts } => todo!(),
            VdSynExprData::Fraction {
                command_token_idx,
                numerator,
                denominator,
                denominator_rcurl_token_idx,
            } => todo!(),
            VdSynExprData::Sqrt {
                command_token_idx,
                radicand,
                radicand_rcurl_token_idx,
            } => todo!(),
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(ref error) => todo!(),
        }
    }
}

impl ToVdSem<VdSemSeparator> for VdSynSeparator {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemSeparator {
        match self {
            VdSynSeparator::Base(lx_token_idx_range, vd_base_separator) => {
                VdSemSeparator::Base(lx_token_idx_range, vd_base_separator)
            }
            VdSynSeparator::Composite(arena_idx, vd_composite_separator) => todo!(),
        }
    }
}
