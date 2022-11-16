use husky_entity_path::EntityPathItd;

use crate::*;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstVariant {
    TypeDefnHead {
        ident: RangedCustomIdentifier,
        kind: TyKind,
        spatial_parameters: IdentDict<SpatialParameter>,
    },
    MainDefnHead,
    CallFormDefnHead {
        ident: RangedCustomIdentifier,
        paradigm: Paradigm,
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        return_ty: RawExprIdx,
        output_liason: OutputModifier,
        opt_this_liason: Option<ParameterModifier>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedCustomIdentifier,
        return_ty: RawExprIdx,
    },
    FieldDefnHead {
        liason: MemberModifier,
        ranged_ident: RangedCustomIdentifier,
        field_ty: RawExprIdx,
        ast_field_kind: AstFieldKind,
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
        source_file: PathItd,
    },
    Visual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { entity_path: EntityPathItd },
    All { parent: Ty },
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
