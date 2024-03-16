mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum MajorItemEthTemplate {
    Type(TypeEthTemplate),
    Fugitive(FugitiveEthTemplate),
    Trait(TraitEthTemplate),
}

impl HasEthTemplate for MajorItemPath {
    type EthTemplate = MajorItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        Ok(match self {
            MajorItemPath::Type(path) => path.eth_template(db)?.into(),
            MajorItemPath::Trait(path) => path.eth_template(db)?.into(),
            MajorItemPath::Fugitive(path) => path.eth_template(db)?.into(),
        })
    }
}
