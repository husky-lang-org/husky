use husky_debugger::*;
use __husky_root::__main_utils::*;
use record_examples::__init__::LINKAGES;
use husky_compile_time::*;

#[tokio::main]
async fn main() {
    let code_snapshot_dir =
        "crates/record-examples/snapshot/record-examples".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(SampleId(23)),
            verbose: false,
            warn_missing_linkage: true,
        },
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}
