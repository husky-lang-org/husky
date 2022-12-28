use super::*;

#[salsa::interned(jar = EntityPathJar, override_debug)]
pub struct TraitPath {
    pub module_path: ModulePath,
    pub ident: Identifier,
    pub connection: ModuleItemConnection,
}

impl TraitPath {
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.module_path(db).show_aux(f, db)?;
        f.write_str(show_connection(self.connection(db)))?;
        f.write_str(self.ident(db).data(db))
    }
}

impl<Db: EntityPathDb + ?Sized> salsa::DebugWithDb<Db> for TraitPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        f.write_str("TraitPath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`)")
    }
}
