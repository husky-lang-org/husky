use crate::*;
use code_generator::RustCodeGenerator;

pub(crate) fn rust_init_rs_content(
    db: &dyn RustTranspileDb,
    target_entrance: SourcePath,
) -> Arc<String> {
    todo!()
    // msg_once!("deal with submodules");
    // let mut generator = RustCodeGenerator::new_lib(db, target_entrance, true);
    // generator.gen_init_content();
    // Arc::new(generator.finish())
}
