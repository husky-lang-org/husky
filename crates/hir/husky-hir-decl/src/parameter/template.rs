use crate::*;
use husky_hir_ty::trai::HirTrait;

use husky_syn_expr::{TemplateParameterSyndicateData, TemplateSynParameterData};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar = HirDeclJar)]
pub struct HirTemplateParameter {
    symbol: HirComptimeSymbol,
    data: HirTemplateParameterData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar = HirDeclJar)]
pub enum HirTemplateParameterData {
    Type { ident: Ident, traits: Vec<HirTrait> },
    Constant { ident: Ident, ty: HirType },
    Lifetime { label: Label },
    Place { label: Label },
}

impl HirTemplateParameter {
    pub fn from_syn(
        syndicate: &TemplateSynParameterData,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        let EtherealTerm::Symbol(symbol) = builder.current_syn_symbol_term(syndicate.symbol())
        else {
            todo!()
        };
        let db = builder.db();
        let symbol = HirComptimeSymbol::from_ethereal(symbol, db)?;
        let data = match syndicate.data() {
            TemplateParameterSyndicateData::Type {
                ident_token,
                traits,
            } => HirTemplateParameterData::Type {
                ident: ident_token.ident(),
                traits: match traits {
                    Some(_) =>
                    /* ad hoc */
                    {
                        vec![]
                    }
                    None => vec![],
                },
            },
            &TemplateParameterSyndicateData::Constant {
                const_token: _,
                ident_token,
                colon_token: _,
                ty_expr,
            } => {
                let ident = ident_token.ident();
                if ident.data(db) == "label" {
                    todo!()
                }
                HirTemplateParameterData::Constant {
                    ident,
                    ty: builder.hir_ty(ty_expr),
                }
            }
            TemplateParameterSyndicateData::Lifetime { label_token } => {
                HirTemplateParameterData::Lifetime {
                    label: label_token.label(),
                }
            }
            TemplateParameterSyndicateData::Place { label_token } => {
                HirTemplateParameterData::Place {
                    label: label_token.label(),
                }
            }
        };
        Some(Self { data, symbol })
    }

    pub fn data(&self) -> &HirTemplateParameterData {
        &self.data
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTemplateParameters(SmallVec<[HirTemplateParameter; 2]>);

impl HirTemplateParameters {
    pub fn from_syn(syndicates: &[TemplateSynParameterData], builder: &HirDeclBuilder) -> Self {
        HirTemplateParameters(
            syndicates
                .iter()
                .filter_map(|syndicate| HirTemplateParameter::from_syn(syndicate, builder))
                .collect(),
        )
    }
}

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
