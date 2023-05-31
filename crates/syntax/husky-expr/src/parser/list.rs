use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum UnfinishedSimpleListOpr {
    NewTuple,
    Index {
        owner: ExprIdx,
    },
    BoxList,
    BoxColonList {
        colon_token_idx: TokenIdx,
    },
    NewLambdaHead,
    FunctionInstantiation {},
    FunctionCall {
        function: ExprIdx,
    },
    RitchieArguments {
        ritchie_kind_token_idx: TokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LeftParenthesisToken,
    },
    TemplateInstantiation {
        template: ExprIdx,
    },
    MethodInstantiation {
        self_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
    },
    MethodCall {
        self_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<ImplicitArgumentList>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ListStartAttr {
    None,
    Attach,
    MethodAttach { ranged_ident: RangedIdent },
}

impl ListStartAttr {
    pub fn attached(&self) -> bool {
        match self {
            ListStartAttr::None => false,
            ListStartAttr::Attach => true,
            ListStartAttr::MethodAttach { .. } => true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListEndAttr {
    None,
    Attach,
    Modulo,
}
