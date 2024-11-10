pub mod parameter;
pub mod table;

use self::parameter::LxCommandParameters;
use crate::path::LxCommandPath;
use latex_math_letter::LxMathLetterStyle;
use parameter::LxCommandParameter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxCommandSignature {
    Complete(LxCompleteCommandSignature),
    MathLetterStyle(LxMathLetterStyle),
    Begin,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LxCompleteCommandSignature {
    path: LxCommandPath,
    parameters: LxCommandParameters,
}

impl LxCompleteCommandSignature {
    pub fn path(&self) -> LxCommandPath {
        self.path
    }

    pub fn parameters(&self) -> &[LxCommandParameter] {
        &self.parameters
    }
}
