use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_defn::*;
use husky_entity_path::EntityPath;
use husky_expr::*;

pub(crate) struct InferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    entity_tree_presheet: &'a EntityTreePresheet,
    entity_tree_sheet: &'a EntityTreeSheet,
    defn_sheet: DefnSheet<'a>,
    module_symbol_context: ModuleSymbolContext<'a>,
    sheet: TokenInfoSheet,
}

impl<'a> InferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInfoDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let token_sheet_data = &db.token_sheet_data(module_path)?;
        Ok(Self {
            db,
            token_sheet_data,
            defn_sheet: db.collect_defns(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            entity_tree_presheet: db.entity_tree_presheet(module_path)?,
            entity_tree_sheet: db.entity_tree_sheet(module_path)?,
            sheet: TokenInfoSheet::new(token_sheet_data),
            module_symbol_context: db.module_symbol_context(module_path)?,
        })
    }

    pub(crate) fn visit_all(mut self) -> TokenInfoSheet {
        self.visit_defns();
        self.visit_use_expr_rules();
        self.sheet
    }

    fn visit_defns(&mut self) {
        for (_, defn) in self.defn_sheet.defns() {
            if let Ok(defn) = defn {
                todo!()
                // self.visit_defn(defn);
            }
        }
    }

    fn visit_use_expr_rules(&mut self) {
        for (rule_idx, rule) in self.entity_tree_sheet.use_expr_rule_indexed_iter() {
            self.visit_use_expr_rule(rule, rule_idx);
        }
    }

    fn visit_use_expr_rule(&mut self, rule: &UseExprRule, rule_idx: UseExprRuleIdx) {
        let use_expr_idx = rule.use_expr_idx();
        let use_expr = &self.entity_tree_presheet[use_expr_idx];
        match use_expr {
            UseExpr::All { star_token } => self
                .sheet
                .add(star_token.token_idx(), TokenInfo::UseExprStar),
            UseExpr::Leaf { ident_token } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::UseExpr {
                    use_expr_idx,
                    rule_idx,
                    state: rule.state(),
                },
            ),
            UseExpr::Parent {
                parent_name_token, ..
            } => self.sheet.add(
                parent_name_token.token_idx(),
                TokenInfo::UseExpr {
                    use_expr_idx,
                    rule_idx,
                    state: rule.state(),
                },
            ),
            UseExpr::Err(_) => (),
            UseExpr::SelfOne { self_token } => todo!(),
        }
    }

    fn visit_defn(&mut self, defn: Defn) {
        let decl = defn.decl(self.db);
        self.visit_expr_region(decl.expr_region(self.db).into());
        defn.expr_region(self.db)
            .map(|expr_region| self.visit_expr_region(expr_region.into()));
        let ast_idx = defn.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                entity_path,
                ident_token,
                is_generic,
                body_kind,
                saved_stream_state,
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::Entity(decl.path(self.db), Some(entity_kind)),
            ),
            Ast::Impl { .. } => (),
            _ => unreachable!(),
        }
        match defn {
            Defn::Type(defn) => self.visit_ty(defn),
            Defn::Trait(defn) => self.visit_trai(defn),
            Defn::Form(defn) => self.visit_form(defn),
            Defn::AssociatedItem(defn) => self.visit_associated_item(defn),
            Defn::Variant(_) => todo!(),
            Defn::ImplBlock(_) => (),
        }
    }

    fn visit_expr_region(&mut self, expr_region: ExprRegion) {
        AuxInferEngine {
            db: self.db,
            token_sheet_data: self.token_sheet_data,
            ast_sheet: self.ast_sheet,
            sheet: &mut self.sheet,
            symbol_context: ExprContext::new(self.db, self.module_symbol_context, expr_region),
            expr_region,
        }
        .visit_all()
    }

    fn visit_ty(&mut self, defn: TypeDefn) {
        match defn {
            TypeDefn::Enum(defn) => self.visit_enum_ty(defn),
            TypeDefn::Inductive(defn) => self.visit_inductive_ty(defn),
            TypeDefn::Record(defn) => self.visit_record_ty(defn),
            TypeDefn::UnitStruct(defn) => self.visit_unit_struct_ty(defn),
            TypeDefn::TupleStruct(defn) => self.visit_tuple_struct_ty(defn),
            TypeDefn::RegularStruct(defn) => self.visit_props_struct_ty(defn),
            TypeDefn::Structure(defn) => self.visit_structure_ty(defn),
            TypeDefn::Alien(defn) => self.visit_alias_ty(defn),
            TypeDefn::Union(_) => todo!(),
        }
    }

    fn visit_enum_ty(&mut self, defn: EnumTypeDefn) {
        // todo!()
    }

    fn visit_inductive_ty(&mut self, defn: InductiveTypeDefn) {
        // todo!()
    }

    fn visit_record_ty(&mut self, defn: RecordTypeDefn) {
        // todo!()
    }

    fn visit_unit_struct_ty(&mut self, defn: UnitStructTypeDefn) {
        // todo!()
    }

    fn visit_tuple_struct_ty(&mut self, defn: TupleStructTypeDefn) {
        // todo!()
    }

    fn visit_props_struct_ty(&mut self, defn: RegularStructTypeDefn) {
        // todo!()
    }

    fn visit_structure_ty(&mut self, defn: StructureTypeDefn) {
        // todo!()
    }

    fn visit_alias_ty(&mut self, defn: AlienTypeDefn) {
        // todo!()
    }

    fn visit_trai(&mut self, defn: TraitDefn) {
        //todo!()
    }

    fn visit_form(&mut self, defn: FormDefn) {
        match defn {
            FormDefn::Function(defn) => self.visit_function(defn),
            FormDefn::Feature(defn) => self.visit_feature(defn),
            FormDefn::Morphism(defn) => self.visit_morphism(defn),
            FormDefn::Value(defn) => self.visit_value(defn),
        }
    }

    fn visit_function(&mut self, defn: FunctionDefn) {}

    fn visit_feature(&mut self, defn: FeatureDefn) {}

    fn visit_morphism(&mut self, defn: MorphismDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_value(&mut self, defn: ValueDefn) {
        let decl = defn.decl(self.db);
        // todo!()
    }

    fn visit_associated_item(&mut self, defn: AssociatedItemDefn) {
        match defn {
            AssociatedItemDefn::TypeItem(defn) => self.visit_ty_item(defn),
            AssociatedItemDefn::TraitItem(defn) => self.visit_trai_item(defn),
            AssociatedItemDefn::TypeAsTraitItem(defn) => self.visit_ty_as_trai_item(defn),
        }
    }

    fn visit_ty_item(&self, defn: TypeItemDefn) {
        // todo!()
    }

    fn visit_trai_item(&self, defn: TraitItemDefn) {
        // todo!()
    }

    fn visit_ty_as_trai_item(&self, defn: TypeAsTraitItemDefn) {
        // todo!()
    }
}

struct AuxInferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    symbol_context: ExprContext<'a>,
    sheet: &'a mut TokenInfoSheet,
    expr_region: ExprRegion,
}

impl<'a> AuxInferEngine<'a> {
    fn visit_all(mut self) {
        for expr in self.symbol_context.exprs() {
            self.visit_expr(expr)
        }
        for entity_path_expr in self.symbol_context.entity_path_exprs() {
            self.visit_entity_path_expr(entity_path_expr)
        }
        for (current_symbol_idx, current_symbol) in
            self.symbol_context.indexed_current_symbol_iter()
        {
            self.visit_current_symbol(current_symbol_idx, current_symbol)
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::CurrentSymbol {
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
                ..
            }
            | Expr::FrameVarDecl {
                token_idx,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => self.sheet.add(
                *token_idx,
                TokenInfo::CurrentSymbol {
                    current_symbol_idx: *current_symbol_idx,
                    current_symbol_kind: *current_symbol_kind,
                    expr_region: self.expr_region,
                },
            ),
            Expr::InheritedSymbol {
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
                ..
            } => self.sheet.add(
                *token_idx,
                TokenInfo::InheritedSymbol {
                    inherited_symbol_idx: *inherited_symbol_idx,
                    expr_region: self.expr_region,
                    inherited_symbol_kind: *inherited_symbol_kind,
                },
            ),
            Expr::SelfType(token_idx) => self.sheet.add(*token_idx, TokenInfo::SelfType),
            Expr::SelfValue(token_idx) => self.sheet.add(*token_idx, TokenInfo::SelfValue),
            Expr::Field { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Field)
            }
            Expr::MethodCall { ident_token, .. } => {
                self.sheet.add(ident_token.token_idx(), TokenInfo::Method)
            }
            Expr::Literal(_)
            | Expr::EntityPath { .. }
            | Expr::BinaryOpn { .. }
            | Expr::PrefixOpn { .. }
            | Expr::SuffixOpn { .. }
            | Expr::TemplateInstantiation { .. }
            | Expr::NewTuple { .. }
            | Expr::NewBoxList { .. }
            | Expr::Bracketed { .. }
            | Expr::Err(_)
            | Expr::Block { .. }
            | Expr::RitchieCall { .. }
            | Expr::Be { .. } => (),
            Expr::BoxColon { .. } => (),
            Expr::ApplicationOrRitchieCall { function, .. }
            | Expr::Application { function, .. } => match self.symbol_context[*function] {
                Expr::NewBoxList {
                    caller: None,
                    lbox_token_idx,
                    items,
                    rbox_token_idx,
                } => {
                    self.sheet.add(lbox_token_idx, TokenInfo::BoxPrefix);
                    self.sheet.add(rbox_token_idx, TokenInfo::BoxPrefix)
                }
                Expr::BoxColon {
                    caller: None,
                    lbox_token_idx,
                    colon_token_idx,
                    rbox_token,
                } => {
                    self.sheet.add(lbox_token_idx, TokenInfo::BoxColon);
                    self.sheet.add(colon_token_idx, TokenInfo::BoxColon);
                    self.sheet.add(rbox_token.token_idx(), TokenInfo::BoxColon)
                }
                _ => (),
            },
        }
    }

    fn visit_entity_path_expr(&mut self, entity_path_expr: &EntityPathExpr) {
        match entity_path_expr {
            EntityPathExpr::Root {
                entity_path,
                token_idx,
                ..
            } => self
                .sheet
                .add(*token_idx, TokenInfo::Entity(Some(*entity_path), None)),
            EntityPathExpr::Subentity {
                path: Ok(entity_path),
                ident_token: Ok(ident_token),
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfo::Entity(Some(*entity_path), None),
            ),
            EntityPathExpr::Subentity { .. } => (),
        }
    }

    fn visit_current_symbol(
        &mut self,
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol: &CurrentSymbol,
    ) {
        let current_symbol_kind = current_symbol.kind();
        match current_symbol_kind {
            CurrentSymbolKind::LetVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | CurrentSymbolKind::Parameter {
                pattern_symbol_idx: pattern_symbol,
            } => match self.symbol_context[pattern_symbol] {
                PatternSymbol::Atom(pattern_expr_idx) => {
                    match self.symbol_context[pattern_expr_idx] {
                        PatternExpr::Identifier {
                            ident_token,
                            liason,
                        } => self.sheet.add(
                            ident_token.token_idx(),
                            TokenInfo::CurrentSymbol {
                                current_symbol_idx,
                                expr_region: self.expr_region,
                                current_symbol_kind,
                            },
                        ),
                        _ => unreachable!(),
                    }
                }
            },
            CurrentSymbolKind::FrameVariable(_) => (),
            CurrentSymbolKind::ImplicitParameter {
                implicit_parameter_kind,
            } => match implicit_parameter_kind {
                ImplicitParameterKind::Type { ident_token } => self.sheet.add(
                    ident_token.token_idx(),
                    TokenInfo::CurrentSymbol {
                        current_symbol_idx,
                        expr_region: self.expr_region,
                        current_symbol_kind,
                    },
                ),
            },
        }
    }
}
