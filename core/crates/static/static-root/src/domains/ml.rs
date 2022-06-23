use datasets_static_defn::DATASETS_MODULE_DEFN;
use models_static_defn::MODELS_DEFN;

use super::*;

pub static ML_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ml",
    items: &[&DATASETS_MODULE_DEFN, &MODELS_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: static_dev_src!(),
};
