use crate::*;
use husky_word::Ident;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermSubentity {
    parent: RawTerm,
    ident: Ident,
}

impl RawTermSubentity {
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn RawTermDb,
        _ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl RawTermRewriteCopy for RawTermSubentity {
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        RawTermSubentity::new(db, parent, ident)
    }
}
