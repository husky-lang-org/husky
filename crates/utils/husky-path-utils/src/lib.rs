mod env;
mod error;
mod module_tree;
mod rel;

pub use env::*;
pub use error::*;
pub use module_tree::*;
pub use rel::*;
pub use std::path::{Path, PathBuf};

use husky_check_utils::should_satisfy;
use relative_path::{RelativePath, RelativePathBuf};

pub fn path_has_file_name(path: &Path, name: &str) -> bool {
    path.file_name().map(|s| s.to_string_lossy()) == Some(name.into())
}

pub fn path_file_name_str(path: &Path) -> Option<String> {
    path.file_name().map(|s| s.to_string_lossy().to_string())
}

pub fn path_parent_file_name_str(path: &Path) -> Option<String> {
    if let Some(parent) = path.parent() {
        parent.file_name().map(|s| s.to_string_lossy().to_string())
    } else {
        None
    }
}

pub fn path_has_extension(path: &Path, extension: &str) -> bool {
    path.extension().map(|s| s.to_string_lossy()) == Some(extension.into())
}

pub fn collect_paths(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];
    collect_dirs_aux(dir, &mut paths);
    paths
}

pub fn collect_dirs_aux(dir: &Path, paths: &mut Vec<PathBuf>) {
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() && path.exists() {
                    collect_dirs_aux(&path, paths)
                }
                paths.push(path)
            }
        }
    }
}

pub fn collect_package_dirs_deprecated(dir: &Path) -> Vec<PathBuf> {
    should_satisfy!(dir, |dir: &Path| dir.is_dir());
    let main_path = dir.join("main.hsy");
    if main_path.exists() {
        return vec![dir.to_path_buf()];
    } else {
        let mut pack_paths = vec![];
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let subpath = entry.path();
            if subpath.is_dir() {
                pack_paths.extend(collect_package_dirs_deprecated(&subpath))
            }
        }
        pack_paths
    }
}

pub fn collect_rust_package_dirs(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let dir = dir.as_ref();
    should_satisfy!(&dir, |dir: &Path| dir.is_dir());
    let mut pack_paths = vec![];
    collect_rust_package_dirs_aux(dir, &mut pack_paths);
    pack_paths.sort();
    pack_paths
}

fn collect_rust_package_dirs_aux(dir: impl AsRef<Path>, pack_paths: &mut Vec<PathBuf>) {
    let dir = dir.as_ref();
    let manifest_path = dir.join("Cargo.toml");
    for entry in std::fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            collect_rust_package_dirs_aux(&subpath, pack_paths)
        }
    }
    if manifest_path.exists() {
        pack_paths.push(dir.to_owned())
    }
}

pub fn collect_husky_package_dirs(dir: &Path) -> Vec<PathBuf> {
    should_satisfy!(&dir, |dir: &Path| dir.is_dir());
    let mut pack_paths = vec![];
    collect_husky_package_dirs_aux(dir, &mut pack_paths);
    pack_paths.sort();
    pack_paths
}

fn collect_husky_package_dirs_aux(dir: &Path, pack_paths: &mut Vec<PathBuf>) {
    let manifest_path = dir.join("Corgi.toml");
    for entry in std::fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            collect_husky_package_dirs_aux(&subpath, pack_paths)
        }
    }
    if manifest_path.exists() {
        pack_paths.push(dir.to_owned())
    }
}

pub fn collect_package_relative_dirs(base: &Path) -> Vec<RelativePathBuf> {
    should_satisfy!(&base, |dir: &Path| dir.is_dir());
    let mut pack_paths = vec![];
    let dir = RelativePathBuf::from(".");
    collect_package_relative_dirs_aux(base, &dir, &mut pack_paths);
    pack_paths.sort();
    pack_paths
}

fn collect_package_relative_dirs_aux(
    base: &Path,
    dir: &RelativePath,
    pack_paths: &mut Vec<RelativePathBuf>,
) {
    let manifest_path = dir.join("Corgi.toml");
    for entry in std::fs::read_dir(&dir.to_logical_path(base)).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            collect_package_relative_dirs_aux(
                base,
                &dir.join(subpath.file_name().unwrap().to_str().unwrap()),
                pack_paths,
            )
        }
    }
    if manifest_path.to_logical_path(base).exists() {
        pack_paths.push(dir.to_owned())
    }
}

#[test]
fn collect_package_relative_dirs_works() {
    let cargo_manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let library_dir = cargo_manifest_dir
        .join("../../../library")
        .canonicalize()
        .unwrap();
    expect_test::expect![[r#"
        [
            "./core",
            "./std",
        ]
    "#]]
    .assert_debug_eq(&collect_package_relative_dirs(&library_dir));

    let examples_dir = cargo_manifest_dir
        .join("../../../examples")
        .canonicalize()
        .unwrap();
    expect_test::expect![[r#"
        [
            "./husky-recognizer",
            "./mnist-classifier",
            "./natural-number-game",
        ]
    "#]]
    .assert_debug_eq(&collect_package_relative_dirs(&examples_dir));
}

#[test]
fn collect_package_dirs_works() {
    fn t(dir: &Path) {
        assert_eq!(
            collect_package_relative_dirs(dir)
                .into_iter()
                .map(|rpath| rpath.to_logical_path(dir))
                .collect::<Vec<_>>(),
            collect_husky_package_dirs(dir)
        )
    }
    let cargo_manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    t(&cargo_manifest_dir
        .join("../../../library")
        .canonicalize()
        .unwrap());
    t(&cargo_manifest_dir
        .join("../../../examples")
        .canonicalize()
        .unwrap())
}

pub fn collect_all_source_files(dir: PathBuf) -> Vec<PathBuf> {
    assert!(dir.is_dir());
    let mut source_files = vec![];
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            source_files.extend(collect_all_source_files(subpath))
        } else {
            if subpath.extension().unwrap() == "hsy" {
                source_files.push(subpath)
            }
        }
    }
    source_files
}

pub fn cargo_manifest_dir() -> Result<PathBuf, std::env::VarError> {
    std::env::var("CARGO_MANIFEST_DIR").map(|s| s.into())
}

pub fn derive_library_path_from_cargo_manifest_dir() -> PathUtilsResult<PathBuf> {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut library_parent_dir: &Path = cargo_manifest_dir.as_ref();
    Ok(loop {
        let library_dir = library_parent_dir.join("library");
        if library_dir.exists() {
            break library_dir;
        }
        if let Some(new_library_parent_dir) = library_parent_dir.parent() {
            library_parent_dir = new_library_parent_dir
        } else {
            todo!()
        }
    })
}

pub fn derive_examples_dir_from_cargo_manifest_dir() -> PathBuf {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut parent_dir: &Path = cargo_manifest_dir.as_ref();
    loop {
        let library_dir = parent_dir.join("library");
        if library_dir.exists() {
            break parent_dir.join("examples");
        }
        if let Some(new_parent_dir) = parent_dir.parent() {
            parent_dir = new_parent_dir
        } else {
            todo!()
        }
    }
}

pub fn clear_directory(path: &Path) -> Result<(), std::io::Error> {
    // Get an iterator over the entries in the directory
    let entries = std::fs::read_dir(path)?;

    // Iterate over the entries and delete them
    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // If the entry is a directory, clear it recursively
            clear_directory(&entry_path)?;
            std::fs::remove_dir(entry_path);
        } else {
            // If the entry is a file, delete it
            std::fs::remove_file(entry_path)?;
        }
    }

    Ok(())
}
