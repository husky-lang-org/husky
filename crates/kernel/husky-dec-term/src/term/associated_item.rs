use crate::*;
use husky_coword::Ident;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct AssociatedItemDecTerm {
    parent: DecTerm,
    ident: Ident,
}

impl AssociatedItemDecTerm {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for AssociatedItemDecTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substitution);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        AssociatedItemDecTerm::new(db, parent, ident)
    }
}