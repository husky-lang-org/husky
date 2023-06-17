use super::*;
use crate::ParentUseExpr;
use husky_token::{PathNameToken, TokenIdx};

#[derive(Debug)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub(crate) enum PresheetAction {
    ResolveUseExpr {
        module_path: ModulePath,
        rule_idx: UseExprRuleIdx,
        path_name_token: PathNameToken,
        symbol: EntitySymbol,
    },
    UpdateUseAll {
        module_path: ModulePath,
        rule_idx: UseAllRuleIdx,
    },
    Err {
        module_path: ModulePath,
        rule_idx: UseExprRuleIdx,
        error: EntityTreeError,
    },
}

impl PresheetAction {
    pub(crate) fn module_path(&self) -> ModulePath {
        match self {
            PresheetAction::ResolveUseExpr { module_path, .. }
            | PresheetAction::UpdateUseAll { module_path, .. }
            | PresheetAction::Err { module_path, .. } => *module_path,
        }
    }
}

impl<'a> EntityTreePresheetMut<'a> {
    pub(crate) fn collect_possible_actions(
        &self,
        ctx: EntityTreeSymbolContext<'a, '_>,
        actions: &mut Vec<PresheetAction>,
    ) {
        for (rule_idx, rule) in self.use_expr_rules.indexed_iter() {
            if rule.is_unresolved() {
                let (name_token, symbol) = match rule.parent() {
                    Some(parent) => match rule.variant() {
                        UseExprRuleVariant::Leaf { ident_token }
                        | UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(ident_token),
                            ..
                        } => (
                            (*ident_token).into(),
                            ctx.resolve_subentity(parent, ident_token.ident())
                                .ok_or(*ident_token),
                        ),
                        UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::SelfMod(self_mod_token),
                            children: _,
                        } => {
                            todo!()
                        }
                        UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::Super(_),
                            children: _,
                        } => {
                            todo!()
                        }
                        UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::CrateRoot(_crate_token),
                            children: _,
                        } => {
                            // todo: prevent this in the parsing stage
                            todo!()
                        }
                    },
                    None => match rule.variant() {
                        UseExprRuleVariant::Leaf { ident_token }
                        | UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::Ident(ident_token),
                            ..
                        } => {
                            let ident_token = *ident_token;
                            (
                                ident_token.into(),
                                ctx.resolve_root_ident(ident_token).ok_or(ident_token),
                            )
                        }
                        UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::SelfMod(self_value_token),
                            children: _,
                        } => (
                            (*self_value_token).into(),
                            Ok(EntitySymbol::SelfModule {
                                module_path: self.module_path,
                            }),
                        ),
                        UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::Super(super_token),
                            children: _,
                        } => match self.module_path.parent(ctx.db()) {
                            Some(super_module_path) => (
                                (*super_token).into(),
                                Ok(EntitySymbol::SuperModule {
                                    current_module_path: self.module_path,
                                    super_module_path,
                                }),
                            ),
                            None => todo!(),
                        },
                        UseExprRuleVariant::Parent {
                            parent_name_token: PathNameToken::CrateRoot(crate_token),
                            children: _,
                        } => (
                            (*crate_token).into(),
                            Ok(EntitySymbol::CrateRoot {
                                root_module_path: ctx.crate_root(),
                            }),
                        ),
                    },
                };
                actions.push(match symbol {
                    Ok(symbol) => PresheetAction::ResolveUseExpr {
                        module_path: self.module_path,
                        rule_idx,
                        symbol,
                        path_name_token: name_token,
                    },
                    Err(ident_token) => PresheetAction::Err {
                        module_path: self.module_path,
                        rule_idx,
                        error: OriginalEntityTreeError::UnresolvedRootIdent(ident_token).into(),
                    },
                })
            }
        }
        for (rule_idx, rule) in self.use_all_rules.indexed_iter() {
            if rule.is_unresolved(&ctx) {
                actions.push(PresheetAction::UpdateUseAll {
                    module_path: self.module_path,
                    rule_idx,
                })
            }
        }
    }

    pub(crate) fn resolve_use_expr(
        &mut self,
        db: &dyn EntityTreeDb,
        rule_idx: UseExprRuleIdx,
        name_token: PathNameToken,
        original_symbol: EntitySymbol,
    ) {
        let rule = &mut self.use_expr_rules[rule_idx];
        #[cfg(test)]
        assert!(rule.is_unresolved());
        rule.mark_as_resolved(original_symbol);
        // if !original_symbol.is_visible_from(db, self.module_path) {
        //     self.errors.push(
        //         OriginalEntityTreeError::SymbolExistsButNotAccessible(
        //             name_token.ident_token().unwrap(),
        //         )
        //         .into(),
        //     );
        // }
        let path = original_symbol.path(db);
        match rule.variant() {
            UseExprRuleVariant::Parent {
                parent_name_token: _,
                children,
            } => {
                for use_expr_idx in children {
                    let use_expr = &self.use_expr_arena[use_expr_idx];
                    let rule = &self.use_expr_rules[rule_idx];
                    match use_expr {
                        UseExpr::All { star_token: _ } => match path {
                            EntityPath::Module(path) => {
                                let new_rule = UseAllRule::new(
                                    db,
                                    self,
                                    path,
                                    rule.ast_idx(),
                                    use_expr_idx,
                                    rule.visibility(),
                                );
                                self.use_all_rules.push(new_rule)
                            }
                            EntityPath::ModuleItem(_) => todo!(),
                            EntityPath::AssociatedItem(_) => todo!(),
                            EntityPath::TypeVariant(_) => todo!(),
                            EntityPath::ImplBlock(_) => todo!(),
                        },
                        UseExpr::Leaf { ident_token } => {
                            let new_rule = rule.new_nonroot(
                                use_expr_idx,
                                path,
                                UseExprRuleVariant::Leaf {
                                    ident_token: *ident_token,
                                },
                            );
                            self.use_expr_rules.push(new_rule)
                        }
                        UseExpr::Parent(ParentUseExpr {
                            parent_name_token,
                            scope_resolution_token: _,
                            children: Ok(children),
                        }) => {
                            let new_rule = rule.new_nonroot(
                                use_expr_idx,
                                path,
                                UseExprRuleVariant::Parent {
                                    parent_name_token: *parent_name_token,
                                    children: children.idx_range(),
                                },
                            );
                            self.use_expr_rules.push(new_rule)
                        }
                        UseExpr::Parent(ParentUseExpr {
                            children: Err(_), ..
                        })
                        | UseExpr::Err(_) => (),
                        UseExpr::SelfOne { self_mod_token: _ } => todo!(),
                    }
                }
            }
            UseExprRuleVariant::Leaf { ident_token: _ } => {
                match self
                    .symbol_table
                    .insert(EntitySymbolEntry::new_use_symbol_entry(
                        db,
                        original_symbol,
                        rule,
                    )) {
                    Ok(_) => (),
                    Err(_) => todo!(),
                }
            }
        }
    }

    pub(crate) fn update_use_all(
        &mut self,
        rule_idx: UseAllRuleIdx,
        new_uses: Vec<EntitySymbolEntry>,
        progress: usize,
    ) {
        let rule = &mut self.use_all_rules[rule_idx];
        rule.set_progress(progress);
        match self.symbol_table.extend(new_uses) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }

    pub(crate) fn mark_use_expr_rule_as_erroneous(
        &mut self,
        rule_idx: UseExprRuleIdx,
        error: EntityTreeError,
    ) {
        let rule = &mut self.use_expr_rules[rule_idx];
        self.errors.push(error);
        rule.mark_as_erroneous()
    }

    pub(crate) fn mark_use_all_rule_as_erroneous(
        &mut self,
        rule_idx: UseAllRuleIdx,
        error: EntityTreeError,
    ) {
        let rule = &mut self.use_all_rules[rule_idx];
        self.errors.push(error);
        rule.mark_as_erroneous()
    }
}
