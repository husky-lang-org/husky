use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu0 {
    // library
    core: EntityPathItd,
    std: EntityPathItd,
}

impl EntityPathMenu0 {
    pub(crate) fn new(db: &dyn EntityPathDb) -> Self {
        Self {
            core: db.it_root_entity_path("core"),
            std: db.it_root_entity_path("std"),
        }
    }

    pub fn core(&self) -> EntityPathItd {
        self.core
    }
}

impl dyn EntityPathDb + '_ {
    fn it_root_entity_path(&self, ident: &str) -> EntityPathItd {
        self.it_entity_path(EntityPath {
            opt_parent: Default::default(),
            ident: self.it_ident(ident),
        })
    }
}
