use latex_command::path::menu::{command_path_menu, LxCommandPathMenu};
use latex_math_punctuation::{LxMathPunctationMap, LxMathPunctuation};
use visored_item_path::VdItemPath;

use crate::{
    resolution::{
        command::{VdCompleteCommandResolution, VdCompleteCommandResolutionMap},
        punctuation::VdPunctuationResolution,
    },
    table::VdDefaultResolutionTable,
};

impl VdDefaultResolutionTable {
    pub fn new_standard(db: &salsa::Db) -> Self {
        let punctuation_resolution_map =
            LxMathPunctationMap::new(lx_math_punctuation_standard_resolution);
        let command_resolution_map = standard_command_resolution_map(db);
        Self::new(punctuation_resolution_map, command_resolution_map, db)
    }
}

fn standard_command_resolution_map(
    db: &salsa::Db,
) -> std::collections::HashMap<
    latex_command::path::LxCommandPath,
    crate::resolution::command::VdCompleteCommandResolution,
    rustc_hash::FxBuildHasher,
> {
    let LxCommandPathMenu {
        int,
        sum,
        prod,
        times,
        otimes,
        alpha,
        beta,
        gamma,
        pi,
        sin,
        cos,
        sqrt,
        frac,
        text,
        ..
    } = *command_path_menu(db);
    VdCompleteCommandResolutionMap::from_iter([
        (int, VdCompleteCommandResolution::INT),
        (sum, VdCompleteCommandResolution::SUM),
        (prod, VdCompleteCommandResolution::PROD),
        (times, VdCompleteCommandResolution::TIMES),
        (otimes, VdCompleteCommandResolution::OTIMES),
        (alpha, VdCompleteCommandResolution::ALPHA),
        (beta, VdCompleteCommandResolution::BETA),
        (gamma, VdCompleteCommandResolution::GAMMA),
        (pi, VdCompleteCommandResolution::PI),
        (sin, VdCompleteCommandResolution::Item(VdItemPath::SIN)),
        (cos, VdCompleteCommandResolution::Item(VdItemPath::COS)),
        (sqrt, VdCompleteCommandResolution::Sqrt),
        (frac, VdCompleteCommandResolution::Frac),
        (text, VdCompleteCommandResolution::Text),
    ])
}

fn lx_math_punctuation_standard_resolution(
    punctuation: LxMathPunctuation,
) -> Option<VdPunctuationResolution> {
    match punctuation {
        LxMathPunctuation::Add => Some(VdPunctuationResolution::SEPARATOR_ADD),
        LxMathPunctuation::Sub => Some(VdPunctuationResolution::SUB),
        LxMathPunctuation::Mul => Some(VdPunctuationResolution::SEPARATOR_MUL),
        LxMathPunctuation::Div => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::In => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::NotIn => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Subset => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Superset => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::SubsetEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::SupersetEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::ForAll => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Exists => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::NotExists => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Infinity => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Equals => Some(VdPunctuationResolution::EQ),
        LxMathPunctuation::NotEquals => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::LessThan => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::GreaterThan => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::LessEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::GreaterEq => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::PlusMinus => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Times => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Lpar => Some(VdPunctuationResolution::LPAR),
        LxMathPunctuation::Rpar => Some(VdPunctuationResolution::RPAR),
        LxMathPunctuation::Lbox => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::Rbox => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::EscapedLcurl => Some(VdPunctuationResolution::Todo),
        LxMathPunctuation::EscapedRcurl => Some(VdPunctuationResolution::Todo),
    }
}
