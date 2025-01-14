mod have;
mod show;

use super::*;
use lean_mir_expr::{
    expr::application::LnMirFunc,
    tactic::{LnMirTacticData, LnMirTacticIdxRange},
};
use visored_mir_expr::expr::application::VdMirFunc;
use visored_mir_opr::separator::VdMirBaseSeparator;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

impl VdTranspileToLean<Dense, LnMirTacticIdxRange> for VdMirStmtIdxRange {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<Dense>) -> LnMirTacticIdxRange {
        let mut tactics = Vec::new();
        builder.build_ln_tactics_from_vd_stmts(self, &mut tactics);
        builder.alloc_tactics(tactics)
    }
}

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(crate) fn build_ln_tactics_from_vd_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        let db = self.db();
        let Some((stmt, following_stmts)) = stmts.first_and_others() else {
            return;
        };
        match *self.stmt_arena()[stmt].data() {
            VdMirStmtData::Block { stmts, ref meta } => {
                match meta {
                    VdMirBlockMeta::Environment(lx_environment_path, _, vd_module_path) => todo!(),
                    VdMirBlockMeta::Division(vd_division_level, vd_module_path) => todo!(),
                }
                self.build_ln_tactics_from_vd_stmts(following_stmts, ln_tactics);
            }
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => {
                // Empty - no tactics to add
                // TODO: maybe intro in certain transpilation style?
                self.build_ln_tactics_from_vd_stmts(following_stmts, ln_tactics);
            }
            VdMirStmtData::Assume { .. } => {
                // Empty - no tactics to add
                // TODO: maybe intro in certain transpilation style?
                self.build_ln_tactics_from_vd_stmts(following_stmts, ln_tactics);
            }
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
                hypothesis_chunk_place,
                ..
            } => {
                ln_tactics.push(LnMirTacticData::Let {
                    ident: match *pattern {
                        VdMirPattern::Letter {
                            letter,
                            symbol_local_defn,
                        } => self.mangle_symbol(symbol_local_defn),
                    },
                    ty: None,
                    construction: assignment.to_lean(self),
                });
                self.build_hypothesis_chunk_tactics(hypothesis_chunk_place.unwrap(), ln_tactics);
                self.build_ln_tactics_from_vd_stmts(following_stmts, ln_tactics);
            }
            VdMirStmtData::Goal { prop } => {
                self.build_ln_tactics_from_vd_stmts(following_stmts, ln_tactics);
            }
            VdMirStmtData::Have {
                prop,
                hypothesis_chunk_place,
                ..
            } => {
                self.build_ln_tactic_from_vd_have(
                    stmt,
                    prop,
                    hypothesis_chunk_place.unwrap(),
                    ln_tactics,
                );
                self.build_ln_tactics_from_vd_stmts(following_stmts, ln_tactics);
            }
            VdMirStmtData::Show {
                prop,
                goal_and_hypothesis_chunk_place,
                ..
            } => {
                // Here, we also provide the following stmts to build the tactic.
                ln_tactics.push(self.build_ln_tactic_from_vd_show(prop, following_stmts));
                if let Some((goal, hypothesis_chunk_place)) = goal_and_hypothesis_chunk_place {
                    // ad hoc, let's see if this works
                    self.build_hypothesis_chunk_tactics(
                        hypothesis_chunk_place.unwrap(),
                        ln_tactics,
                    );
                }
            }
            VdMirStmtData::Qed {
                goal_and_hypothesis_chunk_place,
            } => self.build_qed_tactics(
                stmt,
                goal_and_hypothesis_chunk_place
                    .map(|(_, hypothesis_chunk)| hypothesis_chunk.unwrap()),
                ln_tactics,
            ),
        }
    }

    fn build_ln_tactic_from_vd_paragraph(
        &mut self,
        stmts: VdMirStmtIdxRange,
        tactics: &mut Vec<LnMirTacticData>,
    ) {
        self.build_ln_tactics_from_vd_stmts(stmts, tactics);
    }

    fn build_ln_tactic_from_vd_sentence(
        &mut self,
        stmts: VdMirStmtIdxRange,
        tactics: &mut Vec<LnMirTacticData>,
    ) {
        self.build_ln_tactics_from_vd_stmts(stmts, tactics)
    }
}
