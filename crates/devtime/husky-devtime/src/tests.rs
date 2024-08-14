use crate::*;
use husky_path_utils::HuskyLangDevPaths;
use husky_standard_devsoul::StandardDevsoul;
use husky_standard_visual_protocol::figure::StandardFigure;
use husky_trace_protocol::{
    client::test_utils::TestTraceClient,
    server::{test_utils::TestTraceServer, TraceServer},
};
use tracing_test::traced_test;

// it looks ugly, lol
type StandardDevtime = Devtime<StandardDevsoul>;

#[test]
fn devtime_trace_server_works() {
    let dev_paths = HuskyLangDevPaths::new();
    let devtime = StandardDevtime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
    devtime.test_trace_server();
}

#[traced_test]
#[test]
fn devtime_trace_client_works() {
    let dev_paths = HuskyLangDevPaths::new();
    let devtime = StandardDevtime::new(
        &dev_paths.lang_dev_examples_dir().join("mnist-classifier"),
        None,
    )
    .unwrap();
    devtime.test_trace_client();
}