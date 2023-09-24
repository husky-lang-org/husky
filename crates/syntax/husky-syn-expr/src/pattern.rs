mod contract;
mod region;
mod symbol;

use std::ops::ControlFlow;

pub use self::region::*;
pub use self::symbol::*;

use super::*;
use husky_coword::Ident;
use husky_entity_path::{ItemPath, TypeVariantPath};
use husky_print_utils::p;
use idx_arena::{ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;
use parsec::{IsStreamParser, PunctuatedSmallList, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LiteralData {
    NotNanFloat,
    NotNanF32(NotNan<f32>),
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynPatternExpr {
    /// example: `1`
    Literal(LiteralData),
    /// example: `a`
    Ident {
        symbol_modifier_tokens: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
    },
    /// example: `A::B`
    TypeVariant {
        path_expr_idx: PrincipalEntityPathExprIdx,
        path: TypeVariantPath,
    },
    /// example: `(a, b)`
    Tuple {
        name: Option<ItemPath>,
        fields: SynPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        name: Option<ItemPath>,
        // todo: change to punctuated
        fields: SynPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf {
        options:
            PunctuatedSmallList<SynPatternComponent, VerticalRegionalToken, SynExprError, false, 3>,
    },
    /// example: `x @ 1..9`
    Binding {
        ident_token: IdentRegionalToken,
        asperand_token: AtRegionalToken,
        /// example: `1..9`
        src: SynPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: SynPatternExprIdx,
        dot_dot_token: DotDotRegionalToken,
        end: SynPatternExprIdx,
    },
}

pub type SynPatternExprArena = Arena<SynPatternExpr>;
pub type SynPatternExprIdx = ArenaIdx<SynPatternExpr>;
pub type SynPatternExprIdxRange = ArenaIdxRange<SynPatternExpr>;
pub type SynPatternExprMap<V> = ArenaMap<SynPatternExpr, V>;
pub type SynPatternExprOrderedMap<V> = ArenaOrderedMap<SynPatternExpr, V>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynPatternRoot(SynPatternExprIdx);

impl SynPatternRoot {
    pub fn syn_pattern_expr_idx(self) -> SynPatternExprIdx {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynPatternComponent(SynPatternExprIdx);

impl SynPatternComponent {
    pub fn syn_pattern_expr_idx(self) -> SynPatternExprIdx {
        self.0
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for SynPatternRoot
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let punctuated_patterns = sp.try_parse::<PunctuatedSmallList<
            SynPatternComponent,
            VerticalRegionalToken,
            SynExprError,
            false,
            3,
        >>()?;
        match punctuated_patterns.elements().len() {
            0 => Ok(None),
            1 => Ok(Some(SynPatternRoot(punctuated_patterns.elements()[0].0))),
            _ => todo!(),
        }
    }
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for SynPatternComponent
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        parser: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        let symbol_modifier_tokens: Option<EphemSymbolModifierRegionalTokens> =
            parser.try_parse_option()?;
        let Some((regional_token_idx, token_data)) = parser.next_indexed() else {
            return Ok(None);
        };
        let expr = match parser.disambiguate_token(regional_token_idx, token_data) {
            ControlFlow::Continue(resolved_token) => match resolved_token {
                DisambiguatedTokenData::Literal(syn_expr, _) => todo!(),
                DisambiguatedTokenData::IdentifiableEntityPath(syn_expr) => match syn_expr {
                    IdentifiableEntityPathExpr::Principal {
                        path_expr_idx,
                        opt_path,
                    } => match opt_path {
                        Some(path) => match path {
                            PrincipalEntityPath::Module(_) => {
                                match parser.context().syn_principal_entity_path_expr_arena()
                                    [path_expr_idx]
                                {
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token,
                                        principal_entity_path,
                                    } => match path_name_token {
                                        PathNameRegionalToken::Ident(ident_token) => {
                                            Some(SynPatternExpr::Ident {
                                                symbol_modifier_tokens,
                                                ident_token,
                                            })
                                        }
                                        PathNameRegionalToken::CrateRoot(_) => todo!(),
                                        PathNameRegionalToken::SelfMod(_) => todo!(),
                                        PathNameRegionalToken::Super(_) => todo!(),
                                    },
                                    PrincipalEntityPathExpr::Subitem {
                                        parent,
                                        colon_colon_token,
                                        ref ident_token,
                                        ref path,
                                    } => todo!(),
                                }
                            }
                            PrincipalEntityPath::MajorItem(_) => todo!(),
                            PrincipalEntityPath::TypeVariant(path) => {
                                if symbol_modifier_tokens.is_some() {
                                    todo!()
                                }
                                Some(SynPatternExpr::TypeVariant {
                                    path_expr_idx,
                                    path,
                                })
                            }
                        },
                        None => todo!(),
                    },
                    IdentifiableEntityPathExpr::AssociatedItem {
                        parent_expr_idx,
                        parent_path,
                        colon_colon_regional_token,
                        ident_token,
                    } => todo!(),
                },
                DisambiguatedTokenData::InheritedSymbol { .. } => todo!(),
                DisambiguatedTokenData::CurrentSymbol { .. } => todo!(),
                DisambiguatedTokenData::SelfType(RegionalTokenIdx) => todo!(),
                DisambiguatedTokenData::SelfValue(RegionalTokenIdx) => todo!(),
                DisambiguatedTokenData::Bra(_, _) => todo!(),
                DisambiguatedTokenData::UnrecognizedIdent {
                    regional_token_idx,
                    ident,
                } => Some(SynPatternExpr::Ident {
                    symbol_modifier_tokens,
                    ident_token: IdentRegionalToken::new(ident, regional_token_idx),
                }),
                _ => None,
            },
            ControlFlow::Break(_) => None,
        };
        let Some(expr) = expr else {
            if let Some(symbol_modifier_token_group) = symbol_modifier_tokens {
                todo!()
            } else {
                return Ok(None);
            }
        };
        Ok(Some(SynPatternComponent(
            parser.context.borrow_mut().alloc_pattern_expr(expr),
        )))
    }
}

// pub fn parse_pattern_expr(
//     &mut self,
//     env: SynPatternExprEnvironment,
// ) -> SynExprResult<Option<SynPatternExprIdx>> {
//     todo!("overhaul this");
//     let symbol_modifier_token_group = self.try_parse_option()?;
//     let ident_token = match symbol_modifier_token_group {
//         None => match self.try_parse_option::<IdentRegionalToken>()? {
//             Some(ident_token) => ident_token,
//             None => return Ok(None),
//         },
//         Some(ephem_symbol_modifier_token_group) => {
//             self.try_parse_expected(|token_stream_state| {
//                 OriginalSynExprError::ExpectedIdentAfterModifier(
//                     token_stream_state,
//                     ephem_symbol_modifier_token_group,
//                 )
//             })?
//         }
//     };
//     Ok(Some(self.context_mut().alloc_pattern_expr(
//         SynPatternExpr::Ident {
//             symbol_modifier_keyword_group: symbol_modifier_token_group,
//             ident_token,
//         },
//         env,
//     )))
//     // if let Some(ref_token) = self.parse::<RefToken>()? {
//     //     todo!()
//     // } else if let Some(mut_token) = self.parse::<MutToken>()? {
//     //     let ident_token: RegionalIdentToken =
//     //         self.parse_expected(OriginalExprError::ExpectedIdentAfterMut)?;
//     //     Ok(Some(self.alloc_pattern_expr(
//     //         PatternExpr::Ident {
//     //             ident_token,
//     //             modifier: SymbolModifierKeywordGroup::Mut(mut_token),
//     //         },
//     //         env,
//     //     )))
//     // } else if let Some(ident_token) = self.parse::<RegionalIdentToken>()? {
//     //     Ok(Some(self.alloc_pattern_expr(
//     //         PatternExpr::Ident {
//     //             ident_token,
//     //             modifier: SymbolModifierKeywordGroup::Pure,
//     //         },
//     //         env,
//     //     )))
//     // } else {
//     //     Ok(None)
//     // }
// }
