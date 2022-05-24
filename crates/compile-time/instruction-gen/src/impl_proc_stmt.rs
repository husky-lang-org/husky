use crate::*;

use avec::Avec;
use vm::{
    EagerContract, Instruction, InstructionVariant, VMConditionBranch, VMLoopKind, VMPatternBranch,
};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_proc_stmts(&mut self, stmts: &[Arc<ProcStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_proc_stmt(stmt.clone()));
    }

    fn compile_proc_stmt(&mut self, stmt: Arc<ProcStmt>) {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
                ..
            } => {
                self.compile_eager_expr(initial_value);
                self.def_variable(varname.ident)
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.compile_eager_expr(condition);
                self.push_instruction(Instruction::new(InstructionVariant::Assert, stmt))
            }
            ProcStmtVariant::Return { ref result } => {
                self.compile_eager_expr(result);
                self.push_instruction(Instruction::new(InstructionVariant::Return, stmt));
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.compile_eager_expr(expr);
            }
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => self.compile_loop(loop_variant, stmt.clone(), stmts),
            ProcStmtVariant::Break => {
                self.push_instruction(Instruction::new(InstructionVariant::Break, stmt))
            }
            ProcStmtVariant::ConditionFlow { ref branches, .. } => {
                self.push_instruction(Instruction::new(
                    InstructionVariant::ConditionFlow {
                        branches: self.compile_proc_condition_flow(branches),
                    },
                    stmt,
                ))
            }
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => {
                self.compile_eager_expr(match_expr);
                self.push_instruction(Instruction::new(
                    InstructionVariant::PatternMatch {
                        branches: self.compile_proc_pattern_match(branches),
                    },
                    stmt,
                ))
            }
        }
    }

    fn compile_loop(
        &mut self,
        loop_kind: &LoopVariant,
        loop_stmt: Arc<ProcStmt>,
        body_stmts: &[Arc<ProcStmt>],
    ) {
        match loop_kind {
            LoopVariant::For {
                initial_boundary,
                final_boundary,
                frame_var,
                step,
                ..
            } => {
                self.compile_boundary(initial_boundary, &loop_stmt);
                self.compile_boundary(final_boundary, &loop_stmt);
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.def_variable(frame_var.ident);
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionVariant::Loop {
                        body,
                        loop_kind: VMLoopKind::For {
                            initial_boundary_kind: initial_boundary.kind,
                            final_boundary_kind: final_boundary.kind,
                            step: *step,
                            frame_var: frame_var.ident,
                        },
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::ForExt {
                frame_var,
                final_boundary,
                step,
                ..
            } => {
                self.compile_boundary(final_boundary, &loop_stmt);
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionVariant::Loop {
                        body,
                        loop_kind: VMLoopKind::ForExt {
                            final_boundary_kind: final_boundary.kind,
                            step: *step,
                            frame_var: frame_var.ident,
                            frame_varidx: self.varidx(frame_var.ident),
                        },
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::While { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_eager_expr(condition);
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionVariant::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                block_sheet_builder.compile_proc_stmts(body_stmts);
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionVariant::Loop {
                        body,
                        loop_kind: VMLoopKind::Loop,
                    },
                    loop_stmt,
                ));
            }
            LoopVariant::DoWhile { condition } => {
                let mut block_sheet_builder = self.subsheet_builder();
                block_sheet_builder.compile_proc_stmts(body_stmts);
                block_sheet_builder.compile_eager_expr(condition);
                block_sheet_builder.push_instruction(Instruction::new(
                    InstructionVariant::BreakIfFalse,
                    loop_stmt.clone(),
                ));
                let body = block_sheet_builder.finalize();
                self.push_instruction(Instruction::new(
                    InstructionVariant::Loop {
                        body,
                        loop_kind: VMLoopKind::Loop,
                    },
                    loop_stmt,
                ));
            }
        }
    }

    fn compile_boundary(&mut self, boundary: &Boundary, loop_stmt: &Arc<ProcStmt>) {
        if let Some(ref bound) = boundary.opt_bound {
            self.compile_eager_expr(bound)
        } else {
            self.push_instruction(Instruction::new(
                InstructionVariant::PushPrimitiveLiteral(0i32.into()),
                loop_stmt.clone(),
            ))
        }
    }

    fn compile_proc_condition_flow(
        &self,
        branches: &[Arc<ProcConditionBranch>],
    ) -> Avec<VMConditionBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| match branch.variant {
                    ProcConditionBranchVariant::If { ref condition } => {
                        Arc::new(VMConditionBranch {
                            opt_condition_sheet: {
                                let mut condition_sheet = self.subsheet_builder();
                                condition_sheet.compile_eager_expr(condition);
                                Some(condition_sheet.finalize())
                            },
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    ProcConditionBranchVariant::Elif { ref condition } => {
                        Arc::new(VMConditionBranch {
                            opt_condition_sheet: {
                                let mut condition_sheet = self.subsheet_builder();
                                condition_sheet.compile_eager_expr(condition);
                                Some(condition_sheet.finalize())
                            },
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    ProcConditionBranchVariant::Else => Arc::new(VMConditionBranch {
                        opt_condition_sheet: None,
                        body: {
                            let mut body_sheet = self.subsheet_builder();
                            body_sheet.compile_proc_stmts(&branch.stmts);
                            body_sheet.finalize()
                        },
                    }),
                })
                .collect(),
        )
    }

    fn compile_proc_pattern_match(
        &self,
        branches: &[Arc<ProcPatternBranch>],
    ) -> Avec<VMPatternBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| {
                    Arc::new(match branch.variant {
                        ProcPatternBranchVariant::Case { ref pattern } => VMPatternBranch {
                            opt_pattern: Some(pattern.compile()),
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                        ProcPatternBranchVariant::Default => VMPatternBranch {
                            opt_pattern: None,
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_proc_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                    })
                })
                .collect(),
        )
    }
}
