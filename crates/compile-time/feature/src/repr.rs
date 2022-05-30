use avec::Avec;
use file::FilePtr;
use instruction_gen::new_func_instruction_sheet;
use semantics_eager::ProcStmt;
use semantics_entity::DefinitionRepr;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRepr {
    Expr(Arc<FeatureExpr>),
    LazyBlock(Arc<FeatureLazyBlock>),
    FuncBlock(Arc<FeatureFuncBlock>),
    ProcBlock(Arc<FeatureProcBlock>),
}

impl FeatureRepr {
    pub fn feature(&self) -> FeaturePtr {
        match self {
            FeatureRepr::Expr(expr) => expr.feature,
            FeatureRepr::LazyBlock(block) => block.feature,
            FeatureRepr::FuncBlock(block) => block.feature,
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }

    pub fn file(&self) -> FilePtr {
        match self {
            FeatureRepr::Expr(_) => todo!(),
            FeatureRepr::LazyBlock(block) => block.file,
            FeatureRepr::FuncBlock(block) => block.file,
            FeatureRepr::ProcBlock(block) => block.file,
        }
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            FeatureRepr::Expr(_) => todo!(),
            FeatureRepr::LazyBlock(block) => block.range,
            FeatureRepr::FuncBlock(block) => block.range,
            FeatureRepr::ProcBlock(block) => block.range,
        }
    }

    pub fn from_defn(
        db: &dyn FeatureQueryGroup,
        opt_this: Option<FeatureRepr>,
        defn_repr: &DefinitionRepr,
        features: &FeatureUniqueAllocator,
    ) -> Self {
        match defn_repr {
            DefinitionRepr::EagerExpr {} => todo!(),
            DefinitionRepr::LazyBlock { stmts } => {
                FeatureRepr::LazyBlock(FeatureLazyBlock::new(db, opt_this, stmts, &[], features))
            }
            DefinitionRepr::FuncBlock {
                stmts,
                file,
                range,
                route,
            } => FeatureRepr::FuncBlock(Arc::new(FeatureFuncBlock {
                file: *file,
                range: *range,
                eval_id: Default::default(),
                stmts: stmts.clone(),
                instruction_sheet: new_func_instruction_sheet(
                    db.upcast(),
                    [].into_iter(),
                    stmts,
                    opt_this.is_some(),
                ),
                feature: features.alloc(match opt_this {
                    Some(ref this) => Feature::FieldAccess {
                        this: this.feature(),
                        field_ident: route.ident().custom(),
                    },
                    None => Feature::EntityFeature {
                        route: *route,
                        uid: db.entity_uid(*route),
                    },
                }),
                opt_this,
            })),
            DefinitionRepr::ProcBlock { stmts, file, range } => todo!(),
        }
    }
}

impl From<Arc<FeatureExpr>> for FeatureRepr {
    fn from(expr: Arc<FeatureExpr>) -> Self {
        Self::Expr(expr)
    }
}

impl From<Arc<FeatureLazyBlock>> for FeatureRepr {
    fn from(block: Arc<FeatureLazyBlock>) -> Self {
        Self::LazyBlock(block)
    }
}
