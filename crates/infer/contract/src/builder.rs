mod eager;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstKind};
use infer_ty::TySheet;
use word::BuiltinIdentifier;

use crate::*;

pub struct ContractSheetBuilder<'a> {
    db: &'a dyn InferContractSalsaQueryGroup,
    file: FilePtr,
    main_file: FilePtr,
    contract_sheet: ContractSheet,
}

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn new(
        db: &'a dyn InferContractSalsaQueryGroup,
        file: FilePtr,
        ty_sheet: Arc<TySheet>,
    ) -> Self {
        Self {
            db,
            file,
            main_file: db.main_file(file).unwrap(),
            contract_sheet: ContractSheet::new(ty_sheet),
        }
    }

    pub(crate) fn infer_all(&mut self, ast_iter: AstIter) {
        let arena = self.contract_sheet.ty_sheet.ast_text.arena.clone();
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDecl { .. } | AstKind::EnumVariant { .. } => {
                        item.children.map(|children| self.infer_all(children));
                    }
                    AstKind::MainDecl => {
                        let output_ty = self.db.global_output_ty(self.main_file).unwrap();
                        self.infer_morphism(output_ty, item.children.unwrap(), &arena)
                    }
                    AstKind::DatasetConfig => self.infer_routine(
                        BuiltinIdentifier::DatasetType.into(),
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::RoutineDecl {
                        ref routine_head, ..
                    } => self.infer_routine(
                        routine_head.output.scope,
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::PatternDecl => todo!(),
                    AstKind::Use { ident, scope } => todo!(),
                    AstKind::MembVar { .. } => (),
                    AstKind::Stmt(_) => todo!(),
                    AstKind::MembRoutineDecl {
                        ref memb_routine_head,
                        ..
                    } => self.infer_routine(
                        memb_routine_head.output.scope,
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::FeatureDecl { ty, .. } => {
                        self.infer_morphism(ty.scope, item.children.unwrap(), &arena)
                    }
                    AstKind::MembFeatureDecl { ident, ty } => {
                        self.infer_morphism(ty, item.children.unwrap(), &arena)
                    }
                },
                _ => (),
            }
        }
    }

    pub(crate) fn finish(self) -> ContractSheet {
        self.contract_sheet
    }
}
