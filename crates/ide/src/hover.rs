mod render;

#[cfg(test)]
mod tests;

use std::iter;

use either::Either;
use hir::{HasSource, Semantics};
use ide_db::{
    base_db::FileRange,
    defs::Definition,
    helpers::{pick_best_token, FamousDefs},
    FxIndexSet, RootDatabase,
};
use itertools::Itertools;
use syntax::{ast, match_ast, AstNode, SyntaxKind::*, SyntaxNode, SyntaxToken, T};

use crate::{
    doc_links::token_as_doc_comment,
    markup::Markup,
    runnables::{runnable_fn, runnable_mod},
    FileID, FilePosition, NavigationTarget, RangeInfo, Runnable, TryToNav,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub links_in_hover: bool,
    pub documentation: Option<HoverDocFormat>,
}

impl HoverConfig {
    fn markdown(&self) -> bool {
        matches!(self.documentation, Some(HoverDocFormat::Markdown))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverDocFormat {
    Markdown,
    PlainText,
}

#[derive(Debug, Clone)]
pub enum HoverAction {
    Runnable(Runnable),
    Implementation(FilePosition),
    Reference(FilePosition),
    GoToType(Vec<HoverGotoTypeData>),
}

impl HoverAction {
    fn goto_type_from_targets(db: &RootDatabase, targets: Vec<hir::ModuleDef>) -> Self {
        let targets = targets
            .into_iter()
            .filter_map(|it| {
                Some(HoverGotoTypeData {
                    mod_path: render::path(
                        db,
                        it.module(db)?,
                        it.name(db).map(|name| name.to_string()),
                    ),
                    nav: it.try_to_nav(db)?,
                })
            })
            .collect();
        HoverAction::GoToType(targets)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HoverGotoTypeData {
    pub mod_path: String,
    pub nav: NavigationTarget,
}

/// Contains the results when hovering over an item
#[derive(Debug, Default)]
pub struct HoverResult {
    pub markup: Markup,
    pub actions: Vec<HoverAction>,
}

// Feature: Hover
//
// Shows additional information, like the type of an expression or the documentation for a definition when "focusing" code.
// Focusing is usually hovering with a mouse, but can also be triggered with a shortcut.
//
// image::https://user-images.githubusercontent.com/48062697/113020658-b5f98b80-917a-11eb-9f88-3dbc27320c95.gif[]
pub(crate) fn hover(
    db: &RootDatabase,
    FileRange { file_id, range }: FileRange,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    todo!()
}

pub(crate) fn hover_for_definition(
    sema: &Semantics<RootDatabase>,
    file_id: FileID,
    definition: Definition,
    node: &SyntaxNode,
    config: &HoverConfig,
) -> Option<HoverResult> {
    let famous_defs = match &definition {
        Definition::BuiltinType(_) => Some(FamousDefs(sema, sema.scope(node).krate())),
        _ => None,
    };
    if let Some(markup) = render::definition(sema.db, definition, famous_defs.as_ref(), config) {
        let mut res = HoverResult::default();
        res.markup = render::process_markup(sema.db, definition, &markup, config);
        if let Some(action) = show_implementations_action(sema.db, definition) {
            res.actions.push(action);
        }

        if let Some(action) = show_fn_references_action(sema.db, definition) {
            res.actions.push(action);
        }

        if let Some(action) = runnable_action(sema, definition, file_id) {
            res.actions.push(action);
        }

        if let Some(action) = goto_type_action_for_def(sema.db, definition) {
            res.actions.push(action);
        }
        return Some(res);
    }
    None
}

fn hover_ranged(
    file: &SyntaxNode,
    range: syntax::TextRange,
    sema: &Semantics<RootDatabase>,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    let expr_or_pat = file.covering_element(range).ancestors().find_map(|it| {
        match_ast! {
            match it {
                ast::Expr(expr) => Some(Either::Left(expr)),
                ast::Pat(pat) => Some(Either::Right(pat)),
                _ => None,
            }
        }
    })?;
    let res = match &expr_or_pat {
        Either::Left(ast::Expr::TryExpr(try_expr)) => render::try_expr(sema, config, try_expr),
        Either::Left(ast::Expr::PrefixExpr(prefix_expr))
            if prefix_expr.op_kind() == Some(ast::UnaryOp::Deref) =>
        {
            render::deref_expr(sema, config, prefix_expr)
        }
        _ => None,
    };
    let res = res.or_else(|| render::type_info(sema, config, &expr_or_pat));
    res.map(|it| {
        let range = match expr_or_pat {
            Either::Left(it) => it.syntax().text_range(),
            Either::Right(it) => it.syntax().text_range(),
        };
        RangeInfo::new(range, it)
    })
}

fn hover_type_fallback(
    sema: &Semantics<RootDatabase>,
    config: &HoverConfig,
    token: &SyntaxToken,
) -> Option<RangeInfo<HoverResult>> {
    let node = token
        .ancestors()
        .take_while(|it| !ast::Item::can_cast(it.kind()))
        .find(|n| ast::Expr::can_cast(n.kind()) || ast::Pat::can_cast(n.kind()))?;

    let expr_or_pat = match_ast! {
        match node {
            ast::Expr(it) => Either::Left(it),
            ast::Pat(it) => Either::Right(it),
            // If this node is a MACRO_CALL, it means that `descend_into_macros_many` failed to resolve.
            // (e.g expanding a builtin macro). So we give up here.
            ast::MacroCall(_it) => return None,
            _ => return None,
        }
    };

    let res = render::type_info(sema, config, &expr_or_pat)?;
    let range = sema.original_range(&node).range;
    Some(RangeInfo::new(range, res))
}

fn show_implementations_action(db: &RootDatabase, def: Definition) -> Option<HoverAction> {
    fn to_action(nav_target: NavigationTarget) -> HoverAction {
        HoverAction::Implementation(FilePosition {
            file_id: nav_target.file_id,
            offset: nav_target.focus_or_full_range().start(),
        })
    }

    let adt = match def {
        Definition::Trait(it) => return it.try_to_nav(db).map(to_action),
        Definition::DataType(it) => Some(it),
        Definition::SelfType(it) => it.self_ty(db).as_adt(),
        _ => None,
    }?;
    adt.try_to_nav(db).map(to_action)
}

fn show_fn_references_action(db: &RootDatabase, def: Definition) -> Option<HoverAction> {
    match def {
        Definition::Function(it) => it.try_to_nav(db).map(|nav_target| {
            HoverAction::Reference(FilePosition {
                file_id: nav_target.file_id,
                offset: nav_target.focus_or_full_range().start(),
            })
        }),
        _ => None,
    }
}

fn runnable_action(
    sema: &hir::Semantics<RootDatabase>,
    def: Definition,
    file_id: FileID,
) -> Option<HoverAction> {
    match def {
        Definition::Module(it) => runnable_mod(sema, it).map(HoverAction::Runnable),
        Definition::Function(func) => {
            let src = func.source(sema.db)?;
            if src.file_id != file_id.into() {
                cov_mark::hit!(hover_macro_generated_struct_fn_doc_comment);
                cov_mark::hit!(hover_macro_generated_struct_fn_doc_attr);
                return None;
            }

            runnable_fn(sema, func).map(HoverAction::Runnable)
        }
        _ => None,
    }
}

fn goto_type_action_for_def(db: &RootDatabase, def: Definition) -> Option<HoverAction> {
    let mut targets: Vec<hir::ModuleDef> = Vec::new();
    let mut push_new_def = |item: hir::ModuleDef| {
        if !targets.contains(&item) {
            targets.push(item);
        }
    };

    if let Definition::GenericParam(hir::GenericParam::TypeParam(it)) = def {
        it.trait_bounds(db)
            .into_iter()
            .for_each(|it| push_new_def(it.into()));
    } else {
        let ty = match def {
            Definition::Local(it) => it.ty(db),
            Definition::GenericParam(hir::GenericParam::ConstParam(it)) => it.ty(db),
            Definition::Field(field) => field.ty(db),
            Definition::Function(function) => function.ret_type(db),
            _ => return None,
        };

        walk_and_push_ty(db, &ty, &mut push_new_def);
    }

    Some(HoverAction::goto_type_from_targets(db, targets))
}

fn walk_and_push_ty(
    db: &RootDatabase,
    ty: &hir::Type,
    push_new_def: &mut dyn FnMut(hir::ModuleDef),
) {
    ty.walk(db, |t| {
        if let Some(adt) = t.as_adt() {
            push_new_def(adt.into());
        } else if let Some(trait_) = t.as_dyn_trait() {
            push_new_def(trait_.into());
        } else if let Some(traits) = t.as_impl_traits(db) {
            traits.into_iter().for_each(|it| push_new_def(it.into()));
        } else if let Some(trait_) = t.as_associated_type_parent_trait(db) {
            push_new_def(trait_.into());
        }
    });
}

fn dedupe_or_merge_hover_actions(actions: Vec<HoverAction>) -> Vec<HoverAction> {
    let mut deduped_actions = Vec::with_capacity(actions.len());
    let mut go_to_type_targets = FxIndexSet::default();

    let mut seen_implementation = false;
    let mut seen_reference = false;
    let mut seen_runnable = false;
    for action in actions {
        match action {
            HoverAction::GoToType(targets) => {
                go_to_type_targets.extend(targets);
            }
            HoverAction::Implementation(..) => {
                if !seen_implementation {
                    seen_implementation = true;
                    deduped_actions.push(action);
                }
            }
            HoverAction::Reference(..) => {
                if !seen_reference {
                    seen_reference = true;
                    deduped_actions.push(action);
                }
            }
            HoverAction::Runnable(..) => {
                if !seen_runnable {
                    seen_runnable = true;
                    deduped_actions.push(action);
                }
            }
        };
    }

    if !go_to_type_targets.is_empty() {
        deduped_actions.push(HoverAction::GoToType(
            go_to_type_targets.into_iter().collect(),
        ));
    }

    deduped_actions
}
