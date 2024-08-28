use super::*;
use husky_debug_utils::detonate;
use token::{
    delimiter::{LCURL, RCURL},
    keyword::StmtKeyword,
};

pub(super) fn reduce_by_stmt(
    pre_asts: Seq<Option<PreAst>>,
    allocated_asts: Seq<Option<Ast>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Ast>>) {
    let pre_asts_nearest_left2 = pre_asts.nearest_left2();
    let pre_asts_nearest_right2 = pre_asts.nearest_right2();
    let new_stmt_asts =
        new_stmt_ast.apply(pre_asts_nearest_left2, pre_asts, pre_asts_nearest_right2);
    let (pre_asts, new_parents) = reduce_pre_asts_by_stmt(pre_asts, new_stmt_asts);
    let allocated_asts =
        allocate_asts_and_update_parents(allocated_asts, new_stmt_asts, new_parents);
    let pre_asts = update_pre_asts_by_new_asts(pre_asts, new_stmt_asts);
    (pre_asts, allocated_asts)
}

// todo: return, assert
fn new_stmt_ast(
    pre_ast_nearest_left2: Option2<(Idx, PreAst)>,
    pre_ast: Option<PreAst>,
    pre_ast_nearest_right2: Option2<(Idx, PreAst)>,
) -> Option<AstData> {
    let PreAst::Keyword(Keyword::Stmt(kw)) = pre_ast? else {
        return None;
    };
    match kw {
        StmtKeyword::Let => {
            let Some((idx1, PreAst::Ast(ast))) = pre_ast_nearest_right2.first() else {
                return None;
            };
            if let Some((_, pre_ast)) = pre_ast_nearest_right2.second() {
                match pre_ast {
                    PreAst::Keyword(_) => (),
                    PreAst::Opr(_) | PreAst::LeftDelimiter(_) => return None,
                    PreAst::RightDelimiter(_) => (),
                    PreAst::Ast(_) => return None,
                    PreAst::Separator(separator) => match separator {
                        Separator::Comma => return None,
                        Separator::Semicolon => (),
                    },
                }
            }
            let (pattern, initial_value) = match ast {
                AstData::Binary {
                    lopd,
                    opr: BinaryOpr::Assign,
                    ropd,
                } => (lopd, Some(ropd)),
                AstData::Ident(_)
                | AstData::Prefix { .. }
                | AstData::Binary { .. }
                | AstData::Delimited { .. }
                | AstData::Call { .. } => (idx1, None),
                _ => return None,
            };
            Some(AstData::LetInit {
                expr: idx1,
                pattern,
                initial_value,
            })
        }
        StmtKeyword::If => {
            let Some((condition, PreAst::Ast(ast1))) = pre_ast_nearest_right2.first() else {
                return None;
            };
            let Some((
                body,
                PreAst::Ast(AstData::Delimited {
                    left_delimiter: LCURL,
                    right_delimiter: RCURL,
                    ..
                }),
            )) = pre_ast_nearest_right2.second()
            else {
                return None;
            };
            Some(AstData::If { condition, body })
        }
        StmtKeyword::Else => {
            let Some((if_stmt, PreAst::Ast(AstData::If { .. }))) = pre_ast_nearest_left2.first()
            else {
                return None;
            };
            let Some((
                body,
                PreAst::Ast(
                    AstData::Delimited {
                        left_delimiter: LCURL,
                        right_delimiter: RCURL,
                        ..
                    }
                    | AstData::If { .. }
                    | AstData::Else { .. },
                ),
            )) = pre_ast_nearest_right2.first()
            else {
                return None;
            };
            if let Some((_, PreAst::Keyword(Keyword::ELSE))) = pre_ast_nearest_right2.second() {
                return None;
            }
            Some(AstData::Else { if_stmt, body })
        }
    }
}

fn reduce_pre_asts_by_stmt(
    pre_asts: Seq<Option<PreAst>>,
    new_asts: Seq<Option<AstData>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Idx>>) {
    let new_asts_nearest_left = new_asts.nearest_left();
    let new_asts_nearest_right = new_asts.nearest_right();
    reduce_pre_ast_by_stmt
        .apply_enumerated(new_asts_nearest_left, new_asts_nearest_right, pre_asts)
        .decouple()
}

fn reduce_pre_ast_by_stmt(
    idx: Idx,
    new_ast_nearest_left: Option<(Idx, AstData)>,
    new_ast_nearest_right: Option<(Idx, AstData)>,
    pre_ast: Option<PreAst>,
) -> (Option<PreAst>, Option<Idx>) {
    if let Some((idx1, ast)) = new_ast_nearest_left {
        match ast {
            AstData::LetInit { expr, .. } if expr == idx => (None, Some(idx1)),
            AstData::If {
                condition, body, ..
            } if condition == idx || body == idx => (None, Some(idx1)),
            AstData::Else { body, .. } if body == idx => (None, Some(idx1)),
            _ => (pre_ast, None),
        }
    } else if let Some((idx1, AstData::Else { if_stmt, .. })) = new_ast_nearest_right
        && if_stmt == idx
    {
        (None, Some(idx1))
    } else {
        (pre_ast, None)
    }
}

#[test]
fn reduce_n_times_for_stmt_works1() {
    t(
        "let x = 1",
        2,
        expect![[r#"
            [
                `let`: "let x = 1" ✓,
                `x`: "x",
                `=`: "x = 1",
                `1`: "1",
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 }",
        2,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }" ✓,
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 } else { 4 }",
        2,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }" ✓,
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: `else` ✓,
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }" ✓,
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 } else { 4 }",
        3,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else { 4 }" ✓,
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
            ]
        "#]],
    );
    t(
        "1 + if 1 == 2 { 3 } else { 4 }",
        3,
        expect![[r#"
            [
                `1`: "1" ✓,
                `+`: `+` ✓,
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else { 4 }" ✓,
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
            ]
        "#]],
    );
    t(
        "1 + if 1 == 2 { 3 } else { 4 }",
        4,
        expect![[r#"
            [
                `1`: "1",
                `+`: "1 + if 1 == 2 { 3 } else { 4 }" ✓,
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else { 4 }",
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 } else if 1 == 3 { 4 }",
        4,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else if 1 == 3 { 4 }" ✓,
                `if`: "if 1 == 3 { 4 }",
                `1`: "1",
                `==`: "1 == 3",
                `3`: "3",
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 } else if 1 == 3 { 4 }",
        5,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else if 1 == 3 { 4 }" ✓,
                `if`: "if 1 == 3 { 4 }",
                `1`: "1",
                `==`: "1 == 3",
                `3`: "3",
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 } else if 1 == 3 { 4 } else { 5 }",
        4,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else if 1 == 3 { 4 } else { 5 }" ✓,
                `if`: "if 1 == 3 { 4 }",
                `1`: "1",
                `==`: "1 == 3",
                `3`: "3",
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
                `else`: "if 1 == 3 { 4 } else { 5 }",
                `{`: `{`,
                `5`: "5",
                `}`: "{ 5 }",
            ]
        "#]],
    );
    t(
        "if 1 == 2 { 3 } else if 1 == 3 { 4 } else { 5 }",
        5,
        expect![[r#"
            [
                `if`: "if 1 == 2 { 3 }",
                `1`: "1",
                `==`: "1 == 2",
                `2`: "2",
                `{`: `{`,
                `3`: "3",
                `}`: "{ 3 }",
                `else`: "if 1 == 2 { 3 } else if 1 == 3 { 4 } else { 5 }" ✓,
                `if`: "if 1 == 3 { 4 }",
                `1`: "1",
                `==`: "1 == 3",
                `3`: "3",
                `{`: `{`,
                `4`: "4",
                `}`: "{ 4 }",
                `else`: "if 1 == 3 { 4 } else { 5 }",
                `{`: `{`,
                `5`: "5",
                `}`: "{ 5 }",
            ]
        "#]],
    );
}