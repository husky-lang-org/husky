use crate::*;

// ad hoc
#[salsa::interned(db = ValDb, jar = ValJar)]
pub struct ValVersionStamp {}

impl Val {
    pub fn version_stamp(self, db: &::salsa::Db) -> ValVersionStamp {
        ValVersionStamp::new(db)
    }
}
