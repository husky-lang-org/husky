use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> + VfsDb {
    fn root_traces(&self, crate_path: CratePath) -> &[Trace];
}

impl<Db> TraceDb for Db
where
    Db: salsa::DbWithJar<TraceJar> + VfsDb,
{
    fn root_traces(&self, crate_path: CratePath) -> &[Trace] {
        crate::helpers::root_traces(self, crate_path).as_ref()
    }
}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    ValItemTracePath,
    ValItemTrace,
    LazyCallTracePath,
    LazyCallTrace,
    LazyExprTracePath,
    LazyExprTrace,
    LazyStmtTracePath,
    LazyStmtTrace,
    lazy_stmt_associated_expr_traces,
    EagerCallTracePath,
    EagerCallTrace,
    EagerExprTracePath,
    EagerExprTrace,
    EagerStmtTracePath,
    EagerStmtTrace,
    eager_stmt_associated_expr_traces,
    LoopGroupTracePath,
    LoopGroupTrace,
    // helpers
    crate::helpers::root_traces,
);
