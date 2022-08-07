mod config;
mod error;
mod gui;
mod instance;
mod internal;
mod mode;
mod notif;

pub use config::HuskyDebuggerConfig;
use convert_case::{Case, Casing};
pub use error::{DebuggerError, DebuggerResult};
use libloading::Library;
pub use mode::Mode;

use avec::Avec;
use futures::executor::ThreadPool;
use gui::handle_query;
use husky_compile_time::HuskyComptime;
use husky_file::FilePtr;
use husky_print_utils::*;
use husky_root_static_defn::__StaticLinkageKey;
use husky_test_utils::TestResult;
use husky_trace_protocol::*;
use husky_trace_time::HuskyTraceTime;
use instance::*;
use internal::HuskyDebuggerInternal;
use json_result::JsonResult;
use notif::handle_notif;
use path_utils::collect_all_package_dirs;
use std::{
    collections::HashMap,
    convert::Infallible,
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    sync::Arc,
};
use std::{sync::Mutex, time::Instant};
use vm::__Linkage;
use warp::Filter;

type GetLinkagesFromCDylib = unsafe extern "C" fn() -> &'static [(__StaticLinkageKey, __Linkage)];

pub async fn debugger_run(package_dir: PathBuf, verbose: bool) -> DebuggerResult<()> {
    let opt_library = get_library(&package_dir);
    let linkages_from_cdylib: &[(__StaticLinkageKey, __Linkage)] = opt_library
        .as_ref()
        .map(|library| unsafe {
            library
                .get::<GetLinkagesFromCDylib>(b"get_linkages")
                .expect("what")()
        })
        .unwrap_or(&[]);
    let husky_debugger = HuskyDebuggerInstance::new(
        HuskyDebuggerConfig {
            package_dir,
            opt_sample_id: None,
            verbose: false,
            compiled: false,
        },
        linkages_from_cdylib,
    );
    husky_debugger.serve("localhost:51617").await
}

pub async fn debugger_test(packages_dir: PathBuf, verbose: bool) {
    assert!(packages_dir.is_dir());
    let package_dirs = collect_all_package_dirs(&packages_dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        husky_print_utils::CYAN,
        husky_print_utils::RESET,
        package_dirs.len()
    );

    for package_dir in package_dirs {
        println!(
            "\n{}test{} {}",
            husky_print_utils::CYAN,
            husky_print_utils::RESET,
            package_dir.as_os_str().to_str().unwrap(),
        );
        let opt_library = get_library(&package_dir);
        let linkages_from_cdylib: &[(__StaticLinkageKey, __Linkage)] = opt_library
            .as_ref()
            .map(|library| unsafe {
                library
                    .get::<GetLinkagesFromCDylib>(b"get_linkages")
                    .expect("what")()
            })
            .unwrap_or(&[]);
        let husky_debugger = HuskyDebuggerInstance::new(
            HuskyDebuggerConfig {
                package_dir,
                opt_sample_id: Some(SampleId(23)),
                verbose: false,
                compiled: false,
            },
            linkages_from_cdylib,
        );
        finalize(
            husky_debugger
                .serve_on_error("localhost:51617", SampleId(0))
                .await,
        )
    }
}

fn get_library(package_dir: &Path) -> Option<Library> {
    let package_name = package_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_case(Case::Snake);
    let library_release_path = package_dir.join(format!(
        "__rust_gen__/target/release/lib{}.so",
        package_name,
    ));
    if library_release_path.exists() {
        return Some(unsafe { Library::new(library_release_path) }.expect("it should work"));
    }
    let library_debug_path =
        package_dir.join(format!("__rust_gen__/target/debug/lib{}.so", package_name,));
    if library_debug_path.exists() {
        todo!()
    }
    None
}

fn finalize(test_result: TestResult) {
    match test_result {
        TestResult::Success => finalize_success(),
        TestResult::Failure => finalize_failure(),
    }
}

fn finalize_success() {
    println!(
        "    {}result{}: {}success{}",
        husky_print_utils::CYAN,
        husky_print_utils::RESET,
        husky_print_utils::GREEN,
        husky_print_utils::RESET,
    )
}

fn finalize_failure() {
    println!(
        "    {}result{}: {}failure{}",
        husky_print_utils::CYAN,
        husky_print_utils::RESET,
        husky_print_utils::RED,
        husky_print_utils::RESET,
    )
}
