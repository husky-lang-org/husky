use crate::*;
use husky_path_utils::HuskyLangDevPaths;
use husky_standard_devsoul::{StandardDevsoul, StandardPedestal};
use husky_standard_visual_protocol::figure::StandardFigure;
use husky_trace_protocol::{
    client::test_utils::TestTraceClient,
    server::{test_utils::TestTraceServer, TraceServer},
};

// it looks ugly, lol
type StandardDevtime = Devtime<StandardDevsoul<StandardFigure<StandardPedestal>>>;

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
