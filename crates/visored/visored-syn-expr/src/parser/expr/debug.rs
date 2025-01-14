use super::*;
use expr::VdSynExprArenaRef;

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub fn show(&self) -> String {
        let db = self.db();
        format!(
            r#"
content:

{}

stack:

{}"#,
            self.builder.content(),
            self.stack
                .show(self.builder.expr_arena().as_arena_ref(), db)
        )
    }
}
