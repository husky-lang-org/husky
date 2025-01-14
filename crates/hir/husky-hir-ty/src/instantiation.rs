use super::*;
use crate::place_contract_site::HirPlaceContractSite;
use crate::quary::HirContractedQuary;
use context::HirTypeContext;
use husky_eth_term::instantiation::EthInstantiation;
use husky_fly_term::{
    instantiation::{FlyInstantiation, FlyTermSymbolResolution},
    FlyTerms,
};
use path::{major_item::ty::TypePath, ItemPath};
use vec_like::{SmallVecMap, SmallVecPairMap};

/// `HirInstantiation` maps each hir symbol to its hir resolution.
///
/// hir resolution might still contain hir symbols.
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirInstantiation {
    path: ItemPath,
    context: HirTypeContext,
    variable_map: SmallVecPairMap<HirTemplateVariable, HirTermSymbolicVariableResolution, 4>,
    separator: Option<u8>,
}

impl std::ops::Deref for HirInstantiation {
    type Target = [(HirTemplateVariable, HirTermSymbolicVariableResolution)];

    fn deref(&self) -> &Self::Target {
        &self.variable_map
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTermSymbolicVariableResolution {
    Explicit(HirTemplateArgument),
    /// means we don't care about it now
    SelfLifetime,
    SelfContractedQuary(HirContractedQuary),
}
impl HirTermSymbolicVariableResolution {
    fn is_univalent_for_javelin(&self) -> bool {
        match self {
            HirTermSymbolicVariableResolution::Explicit(arg) => match arg {
                HirTemplateArgument::Vacant => true,
                HirTemplateArgument::Type(_) => false,
                HirTemplateArgument::Constant(_) => false,
                HirTemplateArgument::Lifetime(_) => true,
                HirTemplateArgument::ContractedQuary(_) => true,
            },
            HirTermSymbolicVariableResolution::SelfLifetime => true,
            HirTermSymbolicVariableResolution::SelfContractedQuary(_) => true,
        }
    }
}

impl HirInstantiation {
    pub fn from_fly(
        instantiation: &FlyInstantiation,
        place_contract_site: &HirPlaceContractSite,
        db: &::salsa::Db,
        terms: &FlyTerms,
    ) -> Self {
        let (variable_map0, variable_map1) = &instantiation.variable_map_splitted();
        let t = |&(variable, resolution)| match HirTemplateVariable::from_eth(variable, db) {
            Some(variable) => Some((
                variable,
                match resolution {
                    FlyTermSymbolResolution::Explicit(term) => {
                        HirTermSymbolicVariableResolution::Explicit(
                            HirTemplateArgument::from_fly(term, db, terms).expect("some"),
                        )
                    }
                    FlyTermSymbolResolution::SelfLifetime => {
                        HirTermSymbolicVariableResolution::SelfLifetime
                    }
                    FlyTermSymbolResolution::SelfQuary(quary) => {
                        HirTermSymbolicVariableResolution::SelfContractedQuary(
                            HirContractedQuary::from_fly(quary, place_contract_site),
                        )
                    }
                },
            )),
            None => None,
        };
        let mut variable_map: SmallVecMap<
            (HirTemplateVariable, HirTermSymbolicVariableResolution),
            4,
        > = variable_map0.iter().filter_map(t).collect();
        let mut separator: Option<u8> = None;
        match variable_map1 {
            Some(variable_map1) => {
                separator = Some(variable_map.len().try_into().unwrap());
                variable_map
                    .extend(variable_map1.iter().filter_map(t))
                    .unwrap()
            }
            None => (),
        }
        Self {
            path: instantiation.path(),
            variable_map,
            separator,
            context: HirTypeContext::from_fly(instantiation, db),
        }
    }

    pub fn from_eth(eth_instantiation: &EthInstantiation, db: &::salsa::Db) -> Self {
        let (variable_map0, variable_map1) = &eth_instantiation.variable_map_splitted();
        let t = |&(variable, term)| match HirTemplateVariable::from_eth(variable, db) {
            Some(symbol) => Some((
                symbol,
                HirTermSymbolicVariableResolution::Explicit(
                    HirTemplateArgument::from_eth(term, db).expect("some"),
                ),
            )),
            None => None,
        };
        let mut variable_map: SmallVecMap<
            (HirTemplateVariable, HirTermSymbolicVariableResolution),
            4,
        > = variable_map0.iter().filter_map(t).collect();
        let mut separator: Option<u8> = None;
        match variable_map1 {
            Some(symbol_map1) => {
                separator = Some(variable_map.len().try_into().unwrap());
                variable_map
                    .extend(symbol_map1.iter().filter_map(t))
                    .unwrap()
            }
            None => (),
        }
        Self {
            path: eth_instantiation.path(),
            variable_map,
            separator,
            context: HirTypeContext::from_eth(eth_instantiation, db),
        }
    }

    pub fn with_ty_path(&self, path: TypePath, db: &::salsa::Db) -> Self {
        {
            // verify
            let ItemPath::TypeVariant(_, ty_variant_path) = self.path else {
                unreachable!()
            };
            assert_eq!(ty_variant_path.parent_ty_path(db), path.into());
        }
        Self {
            path: path.into(),
            context: self.context.clone(),
            variable_map: self.variable_map.clone(),
            separator: self.separator,
        }
    }
}

impl HirInstantiation {
    pub fn path(&self) -> ItemPath {
        self.path
    }

    pub fn context(&self) -> &HirTypeContext {
        &self.context
    }

    pub fn variable_map(&self) -> &[(HirTemplateVariable, HirTermSymbolicVariableResolution)] {
        self.variable_map.as_ref()
    }

    pub fn contracted_quaries(&self) -> SmallVec<[HirContractedQuary; 2]> {
        self.variable_map
            .iter()
            .filter_map(|&(_, res)| match res {
                HirTermSymbolicVariableResolution::Explicit(_) => None,
                HirTermSymbolicVariableResolution::SelfLifetime => None,
                HirTermSymbolicVariableResolution::SelfContractedQuary(quary) => Some(quary),
            })
            .collect()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn is_univalent_for_javelin(&self) -> bool {
        self.variable_map
            .iter()
            .all(|(_, res)| res.is_univalent_for_javelin())
    }
}
