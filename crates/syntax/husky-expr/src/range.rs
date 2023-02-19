use crate::*;
use husky_print_utils::p;
use husky_token::{HasTokenIdxRange, RangedTokenSheet, TokenIdxRange, TokenSheetData};
use husky_vfs::ModulePath;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb, jar = ExprJar)]
pub struct ExprRangeRegion {
    entity_path_expr_ranges: Vec<TokenIdxRange>,
    pattern_expr_ranges: Vec<TokenIdxRange>,
    expr_ranges: Vec<TokenIdxRange>,
}

#[salsa::tracked(jar = ExprJar, return_ref)]
pub(crate) fn expr_range_region(db: &dyn ExprDb, expr_region: ExprRegion) -> ExprRangeRegion {
    ExprRangeCalculator::new(db, expr_region).calc_all()
}

// #[test]
// fn expr_range_sheet_works() {
//     use tests::*;
//     DB::default().vfs_expect_test_debug_with_db("expr_range_sheet", todo!());
// }

impl std::ops::Index<EntityPathExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_ranges[index.raw()]
    }
}

impl std::ops::Index<PatternExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.raw()]
    }
}

impl std::ops::Index<ExprIdx> for ExprRangeRegion {
    type Output = TokenIdxRange;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

struct ExprRangeCalculator<'a> {
    token_sheet_data: &'a TokenSheetData,
    expr_region_data: &'a ExprRegionData,
    entity_path_expr_ranges: Vec<TokenIdxRange>,
    pattern_expr_ranges: Vec<TokenIdxRange>,
    expr_ranges: Vec<TokenIdxRange>,
    stmt_ranges: Vec<TokenIdxRange>,
}

impl<'a> std::ops::Index<EntityPathExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<&EntityPathExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: &EntityPathExprIdx) -> &Self::Output {
        &self.entity_path_expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<PatternExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<ExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<&ExprIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: &ExprIdx) -> &Self::Output {
        &self.expr_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<StmtIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: StmtIdx) -> &Self::Output {
        &self.stmt_ranges[index.raw()]
    }
}

impl<'a> std::ops::Index<&StmtIdx> for ExprRangeCalculator<'a> {
    type Output = TokenIdxRange;

    fn index(&self, index: &StmtIdx) -> &Self::Output {
        &self.stmt_ranges[index.raw()]
    }
}

impl<'a> ExprRangeCalculator<'a> {
    fn new(db: &'a dyn ExprDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        let path = expr_region_data.path();
        let token_sheet_data = db.token_sheet_data(path.module_path(db)).unwrap();
        ExprRangeCalculator {
            token_sheet_data,
            expr_region_data,
            entity_path_expr_ranges: Default::default(),
            pattern_expr_ranges: Default::default(),
            expr_ranges: Default::default(),
            stmt_ranges: Default::default(),
        }
    }

    fn calc_all(mut self) -> ExprRangeRegion {
        // order matters
        self.entity_path_expr_ranges
            .reserve(self.expr_region_data.entity_path_expr_arena().len());
        for entity_path_expr in self.expr_region_data.entity_path_expr_arena().iter() {
            self.entity_path_expr_ranges
                .push(self.calc_entity_path_expr_range(entity_path_expr))
        }
        self.pattern_expr_ranges
            .reserve(self.expr_region_data.pattern_expr_arena().len());
        for pattern_expr in self.expr_region_data.pattern_expr_arena().iter() {
            self.pattern_expr_ranges
                .push(self.calc_pattern_expr_range(pattern_expr))
        }
        self.expr_ranges
            .reserve(self.expr_region_data.expr_arena().len());
        self.stmt_ranges
            .reserve(self.expr_region_data.stmt_arena().len());
        for expr in self.expr_region_data.expr_arena().iter() {
            let expr_range = self.calc_expr_range(expr);
            self.expr_ranges.push(expr_range)
        }
        ExprRangeRegion {
            entity_path_expr_ranges: self.entity_path_expr_ranges,
            pattern_expr_ranges: self.pattern_expr_ranges,
            expr_ranges: self.expr_ranges,
        }
    }

    fn calc_entity_path_expr_range(&self, expr: &EntityPathExpr) -> TokenIdxRange {
        match expr {
            EntityPathExpr::Root {
                token_idx,
                ident,
                entity_path,
            } => TokenIdxRange::new_single(*token_idx),
            EntityPathExpr::Subentity {
                parent,
                scope_resolution_token,
                ident_token,
                path,
            } => match ident_token {
                Ok(ident_token) => self[parent].to(ident_token.token_idx()),
                Err(_) => self[parent].to(scope_resolution_token.token_idx()),
            },
        }
    }

    fn calc_pattern_expr_range(&self, expr: &PatternExpr) -> TokenIdxRange {
        match expr {
            PatternExpr::Literal(_) => todo!(),
            PatternExpr::Identifier {
                ident_token,
                liason,
            } => match liason {
                PatternLiason::None => TokenIdxRange::new_single(ident_token.token_idx()),
                PatternLiason::Mut => todo!(),
            },
            PatternExpr::Entity(_) => todo!(),
            PatternExpr::Tuple { name, fields } => todo!(),
            PatternExpr::Struct { name, fields } => todo!(),
            PatternExpr::OneOf { options } => todo!(),
            PatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            PatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }

    fn calc_expr_range(&mut self, expr: &Expr) -> TokenIdxRange {
        match expr {
            Expr::Literal(token_idx)
            | Expr::InheritedSymbol { token_idx, .. }
            | Expr::CurrentSymbol { token_idx, .. }
            | Expr::FrameVarDecl { token_idx, .. }
            | Expr::SelfType(token_idx)
            | Expr::SelfValue(token_idx) => TokenIdxRange::new_single(*token_idx),
            Expr::BinaryOpn { lopd, ropd, .. } => self[lopd].join(self[ropd]),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => self[*entity_path_expr],
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => match target {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            },
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => todo!(),
            Expr::SuffixOpn {
                opd,
                opr,
                punctuation_token_idx,
            } => todo!(),
            Expr::ApplicationOrRitchieCall {
                function,
                lpar_token_idx,
                argument,
                rpar_token_idx,
            } => todo!(),
            Expr::RitchieCall {
                function,
                rpar_token_idx,
                ..
            } => self[function].to(*rpar_token_idx),
            Expr::Field {
                owner, ident_token, ..
            } => self[owner].to(ident_token.token_idx()),
            Expr::MethodCall {
                self_argument,
                rpar_token_idx,
                ..
            } => self[self_argument].to(*rpar_token_idx),
            Expr::TemplateInstantiation {
                template,
                implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => self[function].join(self[argument]),
            Expr::Bracketed {
                lpar_token_idx,
                rpar_token_idx,
                ..
            } => TokenIdxRange::new(*lpar_token_idx, *rpar_token_idx),
            Expr::NewTuple {
                lpar_token_idx,
                rpar_token_idx,
                ..
            } => TokenIdxRange::new(*lpar_token_idx, *rpar_token_idx),
            Expr::NewBoxList {
                caller,
                lbox_token_idx,
                rbox_token_idx,
                ..
            } => match caller {
                Some(_) => todo!(),
                None => TokenIdxRange::new(*lbox_token_idx, *rbox_token_idx),
            },
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                rbox_token,
                ..
            } => match caller {
                Some(_) => todo!(),
                None => TokenIdxRange::new(*lbox_token_idx, rbox_token.token_idx()),
            },
            Expr::Block { stmts } => self.calc_stmts_range(*stmts),
            Expr::Err(error) => match error {
                ExprError::Original(error) => match error {
                    OriginalExprError::MismatchingBracket {
                        ket_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::MissingRightAngleBracket {
                        langle_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectRightCurlyBrace(token_idx)
                    | OriginalExprError::ExpectIdentifier(token_idx)
                    | OriginalExprError::ExpectColon(token_idx)
                    | OriginalExprError::ExpectRightParenthesis(token_idx)
                    | OriginalExprError::NoMatchingBra {
                        ket_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::ExpectIdentifierAfterDot(token_idx)
                    | OriginalExprError::NoLeftOperandForBinaryOperator {
                        binary_token_idx: token_idx,
                    }
                    | OriginalExprError::NoRightOperandForBinaryOperator {
                        punctuation_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::NoOperandForPrefixOperator {
                        prefix_token_idx: token_idx,
                        ..
                    }
                    | OriginalExprError::MissingItemBeforeComma {
                        comma_token_idx: token_idx,
                    }
                    | OriginalExprError::MissingItemBeforeBe {
                        be_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectLetVariablePattern(token_idx)
                    | OriginalExprError::ExpectAssignToken(token_idx)
                    | OriginalExprError::MissingInitialValue(token_idx)
                    | OriginalExprError::UnexpectedKeyword(token_idx)
                    | OriginalExprError::MissingResult(token_idx)
                    | OriginalExprError::MissingCondition(token_idx)
                    | OriginalExprError::MissingForExpr(token_idx)
                    | OriginalExprError::ExpectBePattern(token_idx)
                    | OriginalExprError::ExpectParameterPattern(token_idx)
                    | OriginalExprError::UnterminatedList {
                        bra_token_idx: token_idx,
                    }
                    | OriginalExprError::ExpectEolColon(token_idx)
                    | OriginalExprError::ExpectIdentifierAfterMut(token_idx)
                    | OriginalExprError::UnexpectedSheba(token_idx)
                    | OriginalExprError::UnrecognizedIdentifier { token_idx, .. }
                    | OriginalExprError::UnresolvedSubentity { token_idx, .. } => {
                        TokenIdxRange::new_single(*token_idx)
                    }
                    OriginalExprError::MissingBlock(_) => todo!(),
                    OriginalExprError::MissingLetVariablesType(_) => todo!(),
                    OriginalExprError::MissingFieldType(_) => todo!(),
                },
                ExprError::Derived(_) => todo!(),
            },
        }
    }

    fn calc_stmts_range(&mut self, stmts: StmtIdxRange) -> TokenIdxRange {
        for stmt in stmts {
            self.save_stmt_range(stmt);
        }
        self[stmts.start()].join(self[stmts.end() - 1])
    }

    fn save_stmt_range(&mut self, stmt_idx: StmtIdx) {
        let range = self.calc_stmt_range(stmt_idx);
        // after calculation, all the child statements must have already been computed and cached
        // so that self.stmt_ranges.len() is equal to stmt_idx.raw()
        assert_eq!(self.stmt_ranges.len(), stmt_idx.raw());
        self.stmt_ranges.push(range)
    }

    fn calc_stmt_range(&mut self, stmt_idx: StmtIdx) -> TokenIdxRange {
        match self.expr_region_data[stmt_idx] {
            Stmt::Let {
                let_token,
                ref let_variable_pattern,
                ref assign_token,
                ref initial_value, /* todo: other types of let initialization */
                ..
            } => {
                let start = let_token.token_idx();
                let end = if let Ok(initial_value) = initial_value {
                    self[initial_value].end().token_idx()
                } else if let Ok(assign_token) = assign_token {
                    assign_token.token_idx()
                } else if let Ok(let_variable_pattern) = let_variable_pattern {
                    todo!()
                } else {
                    let_token.token_idx() + 1
                };
                TokenIdxRange::new(start, end)
            }
            Stmt::Return {
                return_token,
                ref result,
            } => todo!(),
            Stmt::Require {
                require_token,
                ref condition,
            } => todo!(),
            Stmt::Assert {
                assert_token,
                ref condition,
            } => todo!(),
            Stmt::Break { break_token } => todo!(),
            Stmt::Eval { expr_idx } => self[expr_idx],
            Stmt::ForBetween {
                for_token,
                ref block,
                ..
            } => todo!(),
            Stmt::ForIn {
                for_token,
                ref block,
                ..
            } => todo!(),
            Stmt::ForExt {
                forext_token,
                ref block,
                ..
            } => todo!(),
            Stmt::While {
                while_token,
                ref block,
                ..
            } => todo!(),
            Stmt::DoWhile {
                do_token,
                ref condition,
                ref block,
                ..
            } => todo!(),
            Stmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            Stmt::Match {} => todo!(),
            Stmt::Err(_) => todo!(),
        }
    }
}
