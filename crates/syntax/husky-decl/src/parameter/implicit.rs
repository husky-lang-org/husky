use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub struct ImplicitParameterDeclList {
    langle: LeftAngleBracketOrLessThanToken,
    implicit_parameters: Vec<ImplicitParameterDeclPattern>,
    commas: Vec<CommaToken>,
    decl_list_result: Result<(), DeclExprError>,
    rangle: RightAngleBracketToken,
}

impl ImplicitParameterDeclList {
    pub fn lcurl(&self) -> LeftAngleBracketOrLessThanToken {
        self.langle
    }

    pub fn implicit_parameters(&self) -> &[ImplicitParameterDeclPattern] {
        &self.implicit_parameters
    }

    pub fn commas(&self) -> &[CommaToken] {
        self.commas.as_ref()
    }
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for ImplicitParameterDeclList {
    type Error = DeclExprError;

    fn try_parse_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> DeclExprResult<Option<Self>> {
        let Some(langle) = ctx.try_parse_optional::< LeftAngleBracketOrLessThanToken>()? else {
            return Ok(None)
        };
        let (decls, commas, decl_list_result) = parse_separated_list_expected(
            ctx,
            1,
            OriginalDeclExprError::ExpectedImplicitParameterDecl,
        );
        Ok(Some(Self {
            langle,
            implicit_parameters: decls,
            commas,
            decl_list_result,
            rangle: ctx.parse_expected(|token_stream_state| {
                OriginalDeclExprError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                    langle_token_idx: langle.token_idx(),
                    token_stream_state,
                }
            })?,
        }))
    }
}
