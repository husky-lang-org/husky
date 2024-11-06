use expr::binary::VdSemBinaryDispatch;
use visored_opr::opr::binary::VdBaseBinaryOpr;
use visored_zfc_ty::{menu::vd_zfc_ty_menu, term::literal::VdZfcLiteralData};

use super::*;
#[cfg(test)]
use crate::test_helpers::builder::VdSemExprTestBuilder;
use crate::{
    clause::{VdSemClauseArenaRef, VdSemClauseData, VdSemClauseIdx},
    expr::{VdSemExprArenaRef, VdSemExprData, VdSemExprIdx},
    phrase::{
        noun::VdSemNounPhraseData, VdSemPhrase, VdSemPhraseArenaRef, VdSemPhraseData,
        VdSemPhraseIdx,
    },
    sentence::{VdSemSentenceArenaRef, VdSemSentenceData, VdSemSentenceIdx, VdSemSentenceIdxRange},
};

pub struct VdSemExprLaTeXFormatter<'a> {
    db: &'a ::salsa::Db,
    expr_arena: VdSemExprArenaRef<'a>,
    phrase_arena: VdSemPhraseArenaRef<'a>,
    clause_arena: VdSemClauseArenaRef<'a>,
    sentence_arena: VdSemSentenceArenaRef<'a>,
    result: String,
}

impl<'a> VdSemExprLaTeXFormatter<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        expr_arena: VdSemExprArenaRef<'a>,
        phrase_arena: VdSemPhraseArenaRef<'a>,
        clause_arena: VdSemClauseArenaRef<'a>,
        sentence_arena: VdSemSentenceArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            result: Default::default(),
        }
    }

    pub fn fmt_sentences(&mut self, sentences: VdSemSentenceIdxRange) {
        for sentence_idx in sentences {
            self.fmt_sentence(sentence_idx);
            self.result.push_str("\n\n");
        }
    }

    pub fn fmt_sentence(&mut self, sentence_idx: VdSemSentenceIdx) {
        match self.sentence_arena[sentence_idx] {
            VdSemSentenceData::Clauses { clauses, end } => {
                for (index, clause_idx) in clauses.into_iter().enumerate() {
                    self.fmt_clause(clause_idx);
                    if index < clauses.len() - 1 {
                        self.result.push_str(", ");
                    } else {
                        self.result.push_str(". ");
                    }
                }
            }
        }
    }

    pub fn fmt_clause(&mut self, clause_idx: VdSemClauseIdx) {
        match self.clause_arena[clause_idx] {
            VdSemClauseData::Verb => todo!(),
            _ => todo!(),
        }
    }

    pub fn fmt_phrase(&mut self, phrase_idx: VdSemPhraseIdx) {
        match self.phrase_arena[phrase_idx] {
            VdSemPhraseData::Noun(ref vd_sem_noun_phrase_data) => todo!(),
        }
    }

    fn fmt_noun_phrase(&mut self, noun_phrase: &VdSemNounPhraseData) {
        // Implement noun phrase formatting
        // This is a placeholder implementation
        // self.result.push_str("\\textbf{");
        // self.fmt_expr(noun_phrase.head);
        // self.result.push('}');
        todo!()
    }

    pub fn fmt_expr(&mut self, expr_idx: VdSemExprIdx) {
        let db = self.db;
        match self.expr_arena[expr_idx] {
            VdSemExprData::Literal { literal, .. } => match literal.data(db) {
                VdZfcLiteralData::NaturalNumber(s) => {
                    if self
                        .result
                        .chars()
                        .last()
                        .map_or(false, |c| c.is_alphanumeric())
                    {
                        self.result.push(' ');
                    }
                    self.result.push_str(s);
                }
                VdZfcLiteralData::NegativeInteger(_) => todo!(),
                VdZfcLiteralData::FiniteDecimalRepresentation(_) => {
                    todo!()
                }
                VdZfcLiteralData::SpecialConstant(vd_zfc_special_constant) => todo!(),
            },
            VdSemExprData::Letter { .. } => todo!(),
            VdSemExprData::BaseOpr { .. } => todo!(),
            VdSemExprData::SeparatedList { .. } => todo!(),
            VdSemExprData::Binary {
                lopd,
                opr,
                ropd,
                ref dispatch,
                ..
            } => {
                self.fmt_expr(lopd);
                self.result += todo!();
                // opr.latex_code();
                self.fmt_expr(ropd);
            }
            VdSemExprData::Prefix {
                opr,
                opd,
                ref dispatch,
            } => todo!(),
            VdSemExprData::Suffix {
                opd,
                opr,
                ref dispatch,
            } => todo!(),
            VdSemExprData::Attach { .. } => todo!(),
            VdSemExprData::UniadicChain => todo!(),
            VdSemExprData::VariadicChain => todo!(),
            VdSemExprData::UniadicArray => todo!(),
            VdSemExprData::VariadicArray => todo!(),
            VdSemExprData::LxDelimited { .. } => todo!(),
            VdSemExprData::Delimited { .. } => todo!(),
            VdSemExprData::Fraction {
                command_token_idx,
                denominator_rcurl_token_idx,
            } => todo!(),
            VdSemExprData::Sqrt {
                command_token_idx,
                radicand_rcurl_token_idx,
            } => todo!(),
        }
    }

    pub fn finish(self) -> String {
        self.result
    }
}

// #[test]
// fn latex_fmt_works() {
//     let db = &DB::default();
//     let menu = vd_zfc_ty_menu(db);
//     let mut builder = VdSemExprTestBuilder::new(db);
//     let one = builder.new_expr_checked(
//         VdSemExprData::Literal {
//             literal: menu.one_literal(),
//             dispatch: VdSemLiteralDispatch::Int,
//         },
//         "1",
//     );
//     let one_add_one = builder.new_expr_checked(
//         VdSemExprData::Binary {
//             lopd: one,
//             opr: VdBaseBinaryOpr::Add,
//             ropd: one,
//             dispatch: VdSemBinaryDispatch::IntAdd,
//         },
//         "1+1",
//     );
// }
