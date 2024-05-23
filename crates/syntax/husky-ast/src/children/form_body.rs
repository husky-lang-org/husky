use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FormBody {
    ast_idx_range: AstIdxRange,
}

/// # getters
impl FormBody {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl IsAstChildren for FormBody {
    const ALLOW_STMT: AstResult<()> = Ok(());

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let module_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => Err(OriginalAstError::UnexpectedModUnderForm)?,
            EntityKindKeywordGroup::Ritchie(ritchie_item_kind_token) => {
                MajorFormKind::Ritchie(ritchie_item_kind_token.ritchie_item_kind()).into()
            }
            EntityKindKeywordGroup::AssocRitchie(_, _) => {
                Err(OriginalAstError::UnexpectedStaticFnOutsideImplBlock)?
            }
            EntityKindKeywordGroup::ConceptualEntity(_) => MajorFormKind::Conceptual.into(),
            EntityKindKeywordGroup::MajorType(token) => token.type_kind().into(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => MajorFormKind::TypeAlias.into(),
            EntityKindKeywordGroup::Trait(_) => MajorItemKind::Trait,
            EntityKindKeywordGroup::Val(_) => MajorFormKind::Val.into(),
            EntityKindKeywordGroup::Termic(_) => MajorFormKind::Termic.into(),
            EntityKindKeywordGroup::Static(_) => MajorFormKind::Static.into(),
            EntityKindKeywordGroup::Memo(_) => Err(OriginalAstError::UnexpectedMemoUnderForm)?,
        };
        Ok(EntityKind::MajorItem {
            module_item_kind,
            connection: MajorItemConnectionKind::Disconnected,
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for FormBody {
    type Error = AstError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(parser
            .parse_normal_ast_children_indented::<Self>()
            .map(|children| Self {
                ast_idx_range: children,
            }))
    }
}
