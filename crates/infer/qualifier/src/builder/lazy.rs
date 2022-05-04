use ast::{RawExprArena, RawExprRange, RawExprVariant, RawStmt, RawStmtVariant};
use check_utils::should;
use defn_head::InputPlaceholder;
use entity_kind::EntityKind;
use infer_error::derived_not_none;
use print_utils::p;
use text::TextRanged;
use word::RangedCustomIdentifier;

use super::*;

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn infer_morphism(
        &mut self,
        arena: &RawExprArena,
        inputs: &[InputPlaceholder],
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_contract: OutputContract,
    ) {
        self.add_lazy_inputs(inputs);
        self.infer_lazy_stmts(arena, ast_iter, opt_output_ty, output_contract)
    }

    fn add_lazy_inputs(&mut self, inputs: &[InputPlaceholder]) {
        if inputs.len() > 0 {
            if let None = self
                .qualified_ty_sheet
                .lazy_variable_qualified_tys
                .get(&(inputs[0].ident.ident.into(), inputs[0].ranged_ty.row()))
            {
                for input in inputs {
                    let ty = input.ranged_ty.route;
                    should!(self
                        .qualified_ty_sheet
                        .lazy_variable_qualified_tys
                        .insert(
                            (input.ident.ident.into(), inputs[0].ranged_ty.row()),
                            Ok(LazyQualifiedType::new(
                                LazyQualifier::from_input(input.contract, self.db.is_copyable(ty)),
                                ty,
                            )),
                        )
                        .is_none());
                }
            }
        }
    }

    fn infer_lazy_stmts(
        &mut self,
        arena: &RawExprArena,
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_contract: OutputContract,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => {
                        self.infer_lazy_stmt(arena, stmt, opt_output_ty, output_contract)
                    }
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_lazy_stmts(arena, children, opt_output_ty, output_contract)
            }
        }
    }

    fn infer_lazy_stmt(
        &mut self,
        arena: &RawExprArena,
        stmt: &RawStmt,
        opt_output_ty: Option<EntityRoutePtr>,
        output_contract: OutputContract,
    ) {
        match stmt.variant {
            RawStmtVariant::Loop(_) => todo!(),
            RawStmtVariant::Branch(_) => todo!(),
            RawStmtVariant::Exec(_) => todo!(),
            RawStmtVariant::Init {
                init_kind,
                varname,
                initial_value,
            } => {
                if let Some(qt) = self.infer_lazy_expr(arena, initial_value) {
                    should!(self
                        .qualified_ty_sheet
                        .lazy_variable_qualified_tys
                        .insert(
                            (varname.ident.into(), varname.row()),
                            qt.use_for_init(init_kind)
                        )
                        .is_none());
                }
            }
            RawStmtVariant::Return(expr) => {
                match (opt_output_ty, self.infer_lazy_expr(arena, expr)) {
                    (Some(output_ty), Some(qualified_ty)) => {
                        if !qualified_ty.is_implicitly_convertible_to_output(
                            self.db,
                            output_contract,
                            output_ty,
                        ) {
                            todo!()
                        }
                    }
                    _ => (),
                }
            }
            RawStmtVariant::Assert(_) => todo!(),
            RawStmtVariant::Break => todo!(),
        }
    }

    fn infer_lazy_expr(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> Option<LazyQualifiedType> {
        let qualified_qualified_ty_result = self.lazy_expr(arena, raw_expr_idx);
        let opt_qualified_ty = qualified_qualified_ty_result
            .as_ref()
            .map(|qualified_ty| *qualified_ty)
            .ok();
        self.qualified_ty_sheet
            .lazy_expr_qualified_tys
            .insert(raw_expr_idx, qualified_qualified_ty_result);
        opt_qualified_ty
    }

    fn lazy_expr(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyQualifiedType> {
        let raw_expr = &arena[raw_expr_idx];
        let ty = self.raw_expr_ty(raw_expr_idx)?;
        match raw_expr.variant {
            RawExprVariant::Variable { varname, init_row } => match derived_not_none!(self
                .qualified_ty_sheet
                .lazy_variable_qualified_tys
                .get(&(varname.into(), init_row)))?
            {
                Ok(qt) => Ok(*qt),
                Err(e) => Err(e.derived()),
            },
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
            RawExprVariant::This { ty } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::Entity { route, kind } => match kind {
                EntityKind::Module => todo!(),
                EntityKind::Type(_) => todo!(),
                EntityKind::Trait => todo!(),
                EntityKind::Member(_) => todo!(),
                EntityKind::Routine => todo!(),
                EntityKind::Feature => Ok(LazyQualifiedType::new(
                    LazyQualifier::feature(self.db.is_copyable(ty)),
                    ty,
                )),
                EntityKind::Pattern => todo!(),
                EntityKind::Literal => todo!(),
            },
            RawExprVariant::PrimitiveLiteral(_) => Ok(LazyQualifiedType::new(
                LazyQualifier::Copyable,
                self.raw_expr_ty(raw_expr_idx).unwrap(),
            )),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => {
                self.lazy_opn(arena, raw_expr_idx, opr, opds.clone())
            }
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn lazy_opn(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opr: Opr,
        opds: RawExprRange,
    ) -> InferResult<LazyQualifiedType> {
        match opr {
            Opr::Binary(binary_opr) => self.lazy_binary(arena, raw_expr_idx, opds),
            Opr::Prefix(prefix_opr) => self.lazy_prefix(arena, raw_expr_idx, opds),
            Opr::Suffix(_) => todo!(),
            Opr::List(list_opr) => self.lazy_list(arena, raw_expr_idx, list_opr, opds),
        }
    }

    fn lazy_binary(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<LazyQualifiedType> {
        self.infer_lazy_expr(arena, opds.start);
        self.infer_lazy_expr(arena, opds.start + 1);
        Ok(LazyQualifiedType::new(
            LazyQualifier::Transient,
            self.raw_expr_ty(raw_expr_idx)?,
        ))
    }

    fn lazy_prefix(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<LazyQualifiedType> {
        self.infer_lazy_expr(arena, opds.start);
        Ok(LazyQualifiedType::new(
            LazyQualifier::Transient,
            self.raw_expr_ty(raw_expr_idx)?,
        ))
    }

    fn lazy_list(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        list_opr: ListOpr,
        opds: RawExprRange,
    ) -> InferResult<LazyQualifiedType> {
        match list_opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.lazy_call(arena, expr_idx, opds),
            ListOpr::Index => self.lazy_element_access(arena, expr_idx, opds),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn lazy_call(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<LazyQualifiedType> {
        match arena[total_opds.start].variant {
            RawExprVariant::Entity { route, .. } => {
                let call_decl = self.db.call_decl(route)?;
                let opt_opd_qualified_tys: Vec<_> = ((total_opds.start + 1)..total_opds.end)
                    .into_iter()
                    .map(|opd_idx| self.infer_lazy_expr(arena, opd_idx))
                    .collect();
                match call_decl.output.contract {
                    OutputContract::Transfer => {
                        msg_once!("handle ref");
                        Ok(LazyQualifiedType::new(
                            if self.db.is_copyable(call_decl.output.ty) {
                                LazyQualifier::Copyable
                            } else {
                                LazyQualifier::Transient
                            },
                            call_decl.output.ty,
                        ))
                    }
                    OutputContract::MemberAccess => todo!(),
                }
            }
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix) => match suffix {
                    SuffixOpr::MembAccess(ident) => self.lazy_method(
                        opds.start,
                        ident,
                        (total_opds.start + 1)..total_opds.end,
                        arena,
                        expr_idx,
                    ),
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            _ => {
                p!(arena[total_opds.start].variant);
                todo!()
            }
        }
    }

    fn lazy_element_access(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<LazyQualifiedType> {
        let this_qt = derived_not_none!(self.infer_lazy_expr(arena, total_opds.start))?;
        let this_contract = self.lazy_expr_contract(total_opds.start)?;
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_lazy_expr(arena, opd);
        }
        let element_ty = self.raw_expr_ty(expr_idx)?;
        let qual = if self.db.is_copyable(element_ty) {
            LazyQualifier::Copyable
        } else {
            match this_qt.qual {
                LazyQualifier::Copyable => todo!(),
                LazyQualifier::PureRef => todo!(),
                LazyQualifier::GlobalRef => todo!(),
                LazyQualifier::Transient => todo!(),
            }
        };
        Ok(LazyQualifiedType::new(qual, element_ty))
    }

    fn lazy_method(
        &mut self,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
    ) -> InferResult<LazyQualifiedType> {
        let method_decl = self.method_decl(expr_idx)?;
        self.infer_lazy_expr(arena, this);
        for input in inputs {
            self.infer_lazy_expr(arena, input);
        }
        let qual = match method_decl.output.contract {
            OutputContract::Transfer => {
                if self.db.is_copyable(method_decl.output.ty) {
                    LazyQualifier::Copyable
                } else {
                    LazyQualifier::Transient
                }
            }
            OutputContract::MemberAccess => todo!(),
        };
        Ok(LazyQualifiedType::new(qual, method_decl.output.ty))
    }
}
