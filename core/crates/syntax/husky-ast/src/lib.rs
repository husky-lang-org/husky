mod context;
mod error;
mod expr;
mod field;
mod query;
mod stmt;
mod transform;
mod xml;

use std::sync::Arc;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::*;
pub use expr::*;
pub use field::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::*;
pub use transform::*;
pub use xml::*;

use check_utils::*;
use defn_head::*;
use dev_utils::*;
use entity_kind::*;
use error::*;
use husky_atom::*;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_file::FilePtr;
use husky_liason_semantics::*;
use husky_text::*;
use print_utils::*;
use vm::InitKind;
use word::{CustomIdentifier, IdentDict, Identifier, Paradigm, StmtKeyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub variant: AstVariant,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstVariant {
    TypeDefnHead {
        ident: RangedCustomIdentifier,
        kind: TyKind,
        spatial_parameters: IdentDict<SpatialParameter>,
    },
    MainDefn,
    CallFormDefnHead {
        ident: RangedCustomIdentifier,
        paradigm: Paradigm,
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        output_ty: RangedEntityRoute,
        output_liason: OutputLiason,
        opt_this_liason: Option<ParameterLiason>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedCustomIdentifier,
        ty: RangedEntityRoute,
    },
    FieldDefnHead {
        liason: MemberLiason,
        ranged_ident: RangedCustomIdentifier,
        ty: RangedEntityRoute,
        field_ast_kind: FieldAstKind,
    },
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: RangedCustomIdentifier,
        variant_class: EnumVariantKind,
    },
    Use {
        use_variant: UseVariant,
    },
    Submodule {
        ident: RangedCustomIdentifier,
        source_file: FilePtr,
    },
    Visual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { route: EntityRoutePtr },
    All { parent: EntityRoutePtr },
}

impl From<RawStmt> for Ast {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            variant: AstVariant::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for AstVariant {
    fn from(stmt: RawStmt) -> Self {
        AstVariant::Stmt(stmt)
    }
}
