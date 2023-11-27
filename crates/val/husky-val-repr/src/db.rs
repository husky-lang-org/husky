use crate::*;

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar(
    ValRepr,
    val_repr_val,
    val_item_val_repr,
    ValReprExpansion,
    val_repr_expansion,
);
