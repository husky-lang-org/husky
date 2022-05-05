use std::{collections::HashMap, sync::Arc};

use arena::map::{ArenaKeyQuery, ArenaMap};
use ast::RawExprMap;
use infer_contract::ContractSheet;
use infer_error::derived_not_none;
use print_utils::{p, ps};
use std::fmt::Write;
use test_utils::TestComparable;
use text::Row;
use vec_map::VecPairMap;
use word::Identifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedTySheet {
    pub(crate) eager_variable_qualified_tys:
        VecPairMap<(Identifier, Row), InferResult<EagerQualifiedType>>,
    pub(crate) lazy_variable_qualified_tys:
        VecPairMap<(Identifier, Row), InferResult<LazyQualifiedType>>,
    pub(crate) eager_expr_qualified_tys: RawExprMap<InferResult<EagerQualifiedType>>,
    pub(crate) lazy_expr_qualified_tys: RawExprMap<InferResult<LazyQualifiedType>>,
    pub(crate) contract_sheet: Arc<ContractSheet>,
}

impl QualifiedTySheet {
    pub fn new(contract_sheet: Arc<ContractSheet>) -> Self {
        let arena = &contract_sheet.entity_route_sheet.ast_text.arena;
        Self {
            eager_variable_qualified_tys: Default::default(),
            lazy_variable_qualified_tys: Default::default(),
            eager_expr_qualified_tys: ArenaMap::new(arena),
            lazy_expr_qualified_tys: ArenaMap::new(arena),
            contract_sheet,
        }
    }

    pub fn lazy_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerQualifiedType> {
        todo!()
    }

    pub fn eager_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerQualifiedType> {
        match derived_not_none!(self.eager_expr_qualified_tys.get(raw_expr_idx))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn eager_variable_qualified_ty(
        &self,
        varname: Identifier,
        init_row: Row,
    ) -> InferResult<EagerQualifiedType> {
        match derived_not_none!(self
            .eager_variable_qualified_tys
            .get_entry((varname, init_row)))?
        .1
        {
            Ok(qt) => Ok(qt),
            Err(ref e) => Err(e.derived()),
        }
    }
}

impl TestComparable for QualifiedTySheet {
    fn write_inherent(&self, result: &mut String) {
        result.push_str("eager variable qualified types:\n\n");
        self.contract_sheet
            .entity_route_sheet
            .ast_text
            .write_map_inherently(&self.eager_expr_qualified_tys, 4, result);
        ps!(result);
        todo!()
    }
    // fn print_inherent(&self) -> String {
    //     let mut result = String::new();
    //     result.push_str("eager variable qualified types:\n\n");
    //     for ((ident, row), qt_result) in self.eager_variable_qualified_tys.iter() {
    //         write!(
    //             result,
    //             "    {: <4} {: <20}{:?}\n",
    //             row.0,
    //             ident.as_str(),
    //             qt_result
    //         )
    //         .unwrap()
    //     }
    //     println!("{}", &result);
    //     todo!()
    // }
}
