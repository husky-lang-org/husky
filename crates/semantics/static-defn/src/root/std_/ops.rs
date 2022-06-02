use dev_utils::static_dev_src;
use vm::{MemberLiason, OutputLiason, ParameterLiason};

use crate::*;

pub static STD_OPS_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ops",
    subscopes: &[
        ("Index", &STD_OPS_INDEX_DEFN),
        ("IndexMut", &STD_OPS_INDEX_DEFN),
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_OPS_INDEX_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Index",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Trait {
        base_route: "std::ops::Index",
        generic_parameters: &[StaticGenericPlaceholder {
            name: "Idx",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        members: &[
            EntityStaticDefn {
                name: "Output",
                subscopes: &[],
                variant: EntityStaticDefnVariant::TraitAssociatedType {
                    trai: "std::ops::Index",
                    traits: &[],
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "index",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: ParameterLiason::MemberAccess,
                    input_parameters: &[],
                    output_ty: "This::Output",
                    output_liason: OutputLiason::MemberAccess {
                        member_liason: MemberLiason::Mutable,
                    },
                    generic_parameters: &[],
                    kind: MethodStaticDefnVariant::TraitMethod {
                        opt_default_source: None,
                    },
                },
                dev_src: static_dev_src!(),
            },
        ],
    },
    dev_src: dev_utils::static_dev_src!(),
};
