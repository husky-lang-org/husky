use super::*;
use pattern::VdSynPattern;
use symbol::local_defn::{
    VdSynSymbolLocalDefnBody, VdSynSymbolLocalDefnHead, VdSynSymbolLocalDefnSrc,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VdSynLetPlaceholderResolution {
    pattern: VdSynPattern,
    pattern_expr: VdSynExprIdx,
    ty: VdSynLetClausePlaceholderType,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynLetClausePlaceholderType {
    Expr(VdSynExprIdx),
}

impl VdSynLetPlaceholderResolution {
    pub fn pattern(&self) -> &VdSynPattern {
        &self.pattern
    }

    pub fn pattern_expr(&self) -> VdSynExprIdx {
        self.pattern_expr
    }

    pub fn ty(&self) -> VdSynLetClausePlaceholderType {
        self.ty
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub fn build_let_placeholder_resolution(
        &self,
        pattern_expr: VdSynExprIdx,
        ty: VdSynLetClausePlaceholderType,
    ) -> VdSynLetPlaceholderResolution {
        VdSynLetPlaceholderResolution {
            pattern: self.build_pattern(pattern_expr),
            pattern_expr,
            ty,
        }
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub(crate) fn build_symbols_in_let_placeholder_resolution(
        &mut self,
        clause: VdSynClauseIdx,
        resolution: VdSynLetPlaceholderResolution,
    ) {
        // Order matters!
        self.build_let_clause_placeholder_ty(resolution.ty);
        match resolution.pattern {
            VdSynPattern::Letter {
                token_idx_range,
                letter,
                pattern_expr,
            } => {
                self.define_symbol(
                    VdSynSymbolLocalDefnHead::Letter {
                        token_idx_range,
                        letter,
                    },
                    VdSynSymbolLocalDefnBody::Placeholder,
                    VdSynSymbolLocalDefnSrc::LetPlaceholder(clause),
                );
            }
        }
        self.build_expr(resolution.pattern_expr);
    }

    fn build_let_clause_placeholder_ty(&mut self, ty: VdSynLetClausePlaceholderType) {
        match ty {
            VdSynLetClausePlaceholderType::Expr(expr) => self.build_expr(expr),
        }
    }
}
