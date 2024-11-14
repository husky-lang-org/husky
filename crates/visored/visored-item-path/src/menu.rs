use crate::path::{
    category::VdCategoryPath, function::VdFunctionPath, set::VdSetPath, trai::VdTraitPath,
    trai_item::VdTraitItemPath,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdItemPathMenu {
    // # categories
    pub set: VdCategoryPath,
    pub proposition: VdCategoryPath,
    // # sets
    pub nat: VdSetPath,
    pub rat: VdSetPath,
    pub int: VdSetPath,
    pub real: VdSetPath,
    pub complex: VdSetPath,
    // # functions
    pub sin: VdFunctionPath,
    pub cos: VdFunctionPath,
    // # traits
    pub group: VdTraitPath,
    pub ring: VdTraitPath,
    // # trait items
    pub group_mul: VdTraitItemPath,
    pub abelian_group_add: VdTraitItemPath,
    pub ring_add: VdTraitItemPath,
    pub ring_mul: VdTraitItemPath,
}

impl VdItemPathMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        Self {
            // # categories
            set: VdCategoryPath::SET,
            proposition: VdCategoryPath::PROPOSITION,
            // # sets
            nat: VdSetPath::NATURAL_NUMBER,
            rat: VdSetPath::RATIONAL_NUMBER,
            int: VdSetPath::INTEGER,
            real: VdSetPath::REAL_NUMBER,
            complex: VdSetPath::COMPLEX_NUMBER,
            // # functions
            sin: VdFunctionPath::SIN,
            cos: VdFunctionPath::COS,
            // # traits
            group: VdTraitPath::GROUP,
            ring: VdTraitPath::RING,
            // # trait items
            group_mul: VdTraitItemPath::GROUP_MUL,
            abelian_group_add: VdTraitItemPath::ABELIAN_GROUP_ADD,
            ring_add: VdTraitItemPath::RING_ADD,
            ring_mul: VdTraitItemPath::RING_MUL,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_item_path_menu(db: &::salsa::Db) -> VdItemPathMenu {
    VdItemPathMenu::new(db)
}
