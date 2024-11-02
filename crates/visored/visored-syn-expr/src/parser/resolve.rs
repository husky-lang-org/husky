use super::{
    expr::{VdSynExprClass, VdSynExprData, VdSynExprIdx},
    VdSynExprParser,
};
use latex_ast::ast::math::{LxMathAstData, LxMathAstIdx};
use latex_math_letter::LxMathLetter;
use latex_token::{
    data::math::digit::LxMathDigit,
    idx::{LxMathTokenIdx, LxTokenIdxRange},
};
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_opr::{
    delimiter::{VdBaseLeftDelimiter, VdBaseRightDelimiter},
    opr::VdBaseOpr,
    separator::VdBaseSeparator,
};
use visored_resolution::resolution::punctuation::VdPunctuationResolution;
use visored_zfc_ty::term::literal::{VdZfcLiteral, VdZfcLiteralData};

pub struct DisambiguatedMathAst {
    ast: LxMathAstIdx,
    preceding_space_annotation: Option<VdSpaceAnnotation>,
}

#[derive(Debug)]
pub enum ResolvedToken {
    Expr(VdSynExprData, VdSynExprClass),
    Opr(LxMathTokenIdx, VdBaseOpr),
    Separator(VdBaseSeparator),
    LeftDelimiter(VdBaseLeftDelimiter),
    RightDelimiter(VdBaseRightDelimiter),
}

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub fn resolve_token(&mut self, next: &mut LxMathAstIdx, end: LxMathAstIdx) -> ResolvedToken {
        let ast_data = &self.builder.ast_arena()[*next];
        *next += 1;
        match *ast_data {
            LxMathAstData::Letter(token_idx, letter) => ResolvedToken::Expr(
                VdSynExprData::Letter { token_idx, letter },
                VdSynExprClass::Atom,
            ),
            LxMathAstData::Punctuation(token_idx, punctuation) => {
                if let Some(token_annotation) =
                    self.builder.annotations().token_annotation(*token_idx)
                {
                    return todo!();
                }
                match self.builder.default_resolution_table()[punctuation] {
                    Some(resolution) => match resolution {
                        VdPunctuationResolution::Opr(vd_base_opr) => {
                            ResolvedToken::Opr(token_idx, vd_base_opr)
                        }
                        VdPunctuationResolution::Todo => todo!(),
                    },
                    None => todo!(),
                }
            }
            LxMathAstData::Digit(first_token_idx, digit) => {
                let mut last_token_idx = first_token_idx;
                let mut last_offset_end = self.builder.token_storage()[*first_token_idx].0.end();
                let mut s = String::from(digit.char());
                // TODO: handle real number by using a kind variable, literal number kind
                while *next < end {
                    match self.builder.ast_arena()[*next] {
                        LxMathAstData::Digit(token_idx, digit) => {
                            let offset_range = self.builder.token_storage()[*token_idx].0;
                            if offset_range.start() != last_offset_end {
                                break;
                            }
                            if let Some(space_annotation) = self
                                .builder
                                .annotations()
                                .preceding_space_annotation(*token_idx)
                            {
                                match space_annotation {
                                    VdSpaceAnnotation::Apply(apply_annotation) => todo!(),
                                    VdSpaceAnnotation::Sever(sever_annotation) => todo!(),
                                }
                            }
                            last_token_idx = token_idx;
                            last_offset_end = offset_range.end();
                            s.push(digit.char())
                        }
                        // TODO: handle real number
                        _ => break,
                    }
                    *next += 1;
                }
                let expr_data = VdSynExprData::Literal {
                    token_idx_range: LxTokenIdxRange::new_closed(*first_token_idx, *last_token_idx),
                    literal: VdZfcLiteral::new(
                        VdZfcLiteralData::NaturalNumber(s),
                        self.builder.db(),
                    ),
                };
                ResolvedToken::Expr(expr_data, VdSynExprClass::Atom)
            }
            LxMathAstData::TextEdit { ref buffer } => todo!(),
            LxMathAstData::Attach { base, ref scripts } => todo!(),
            LxMathAstData::Delimited {
                left_delimiter_token_idx,
                left_delimiter,
                asts,
                right_delimiter_token_idx,
                right_delimiter,
            } => todo!(),
            LxMathAstData::Command {
                command_token_idx,
                command_path,
            } => todo!(),
        }
    }
}
