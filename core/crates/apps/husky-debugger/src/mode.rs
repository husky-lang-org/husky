use crate::*;
use __husky_root::__resolve_root_defn;
use path_utils::collect_all_package_dirs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Mode {
    Run,
    Test,
}

impl Mode {
    pub async fn run(&self, dir: PathBuf) {
        match self {
            Mode::Run => run(&dir).await,
            Mode::Test => test_all_packages_in_dir(dir).await,
        }
    }
}

impl From<Option<String>> for Mode {
    fn from(opt_str: Option<String>) -> Self {
        if let Some(ref s) = opt_str {
            match s.as_str() {
                "test" => Mode::Test,
                "run" => Mode::Run,
                _ => panic!(),
            }
        } else {
            Mode::Run
        }
    }
}

async fn run(package_dir: &Path) {
    HuskyDebugger::new_from_flags()
        .serve("localhost:51617")
        .await
        .expect("")
}

async fn test_all_packages_in_dir(dir: PathBuf) {
    assert!(dir.is_dir());
    let package_dirs = collect_all_package_dirs(dir);
    println!(
        "\n{}Running{} tests on {} example packages:",
        print_utils::CYAN,
        print_utils::RESET,
        package_dirs.len()
    );

    for package_dir in package_dirs {
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
            package_dir.as_os_str().to_str().unwrap(),
        );
        match HuskyDebugger::new(
            HuskyDebuggerConfig {
                package_dir,
                opt_sample_id: Some(SampleId(23)),
                verbose: false,
                warn_missing_linkage: false,
            },
            &[],
        )
        .serve_on_error("localhost:51617", SampleId(0))
        .await
        {
            TestResult::Success => finalize_success(),
            TestResult::Failed => finalize_failure(),
        }
    }
}

fn finalize_success() {
    println!(
        "    {}result{}: {}success{}",
        print_utils::CYAN,
        print_utils::RESET,
        print_utils::GREEN,
        print_utils::RESET,
    )
}

fn finalize_failure() {
    println!(
        "    {}result{}: {}failure{}",
        print_utils::CYAN,
        print_utils::RESET,
        print_utils::RED,
        print_utils::RESET,
    )
}
