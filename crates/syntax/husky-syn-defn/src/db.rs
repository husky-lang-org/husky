use crate::*;

use salsa::DbWithJar;

pub trait SynDefnDb: DbWithJar<SynDefnJar> + SynDeclDb {}

impl SynDefnDb for Db where Db: DbWithJar<SynDefnJar> + SynDeclDb {}
