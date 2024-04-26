use super::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::trai_for_ty_impl_block::TraitForTypeImplBlockPath;

#[salsa::tracked]
pub struct TraitForTypeMethodRitchieSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub ritchie_item_kind: RitchieItemKind,
    #[return_ref]
    pub template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    #[return_ref]
    pub parenate_parameter_decl_list: SynNodeDeclResult<ParenateParameterSyndicateList<true>>,
    #[return_ref]
    pub light_arrow_token: TokenDataResult<Option<LightArrowRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeColonSyndicate>>,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeMethodRitchieSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.parenate_parameter_decl_list(db)
                        .as_ref()
                        .err()
                        .into_iter(),
                )
                .chain(self.return_ty(db).as_ref().err().into_iter())
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_trai_for_ty_method_ritchie_node_decl(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
        ritchie_item_kind: RitchieItemKind,
    ) -> TraitForTypeMethodRitchieSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = syn_node_path.data(db).impl_block(db).syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
            None,
        );
        let template_parameter_decl_list = parser.try_parse_option();
        let parenate_parameter_decl_list =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedParameterDeclList);
        let light_arrow_token = parser.try_parse_option();
        let return_ty = if let Ok(Some(_)) = light_arrow_token {
            parser
                .try_parse_expected(OriginalSynNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEolColon);
        TraitForTypeMethodRitchieSynNodeDecl::new(
            db,
            syn_node_path,
            ritchie_item_kind,
            template_parameter_decl_list,
            parenate_parameter_decl_list,
            light_arrow_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked]
pub struct TraitForTypeMethodRitchieSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ritchie_item_kind: RitchieItemKind,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub self_value_parameter: Option<SelfValueParameterSyndicate>,
    #[return_ref]
    pub parenate_parameters: ParenateSynParametersData,
    pub return_ty: Option<ReturnTypeBeforeColonSyndicate>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeMethodRitchieSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeMethodRitchieSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let ritchie_item_kind = syn_node_decl.ritchie_item_kind(db);
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let parenate_parameter_decl_list =
            syn_node_decl.parenate_parameter_decl_list(db).as_ref()?;
        let self_value_parameter = *parenate_parameter_decl_list.self_value_parameter();
        let parenate_parameters: ParenateSynParametersData = parenate_parameter_decl_list
            .parenate_parameters()
            .iter()
            .map(Clone::clone)
            .collect();
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(TraitForTypeMethodRitchieSynDecl::new(
            db,
            path,
            ritchie_item_kind,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
            syn_expr_region,
        ))
    }

    pub fn impl_block_path(self, db: &::salsa::Db) -> TraitForTypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}