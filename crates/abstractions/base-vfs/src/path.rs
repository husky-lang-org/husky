use husky_print_utils::p;
use salsa::Durability;

use super::*;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

/// `VirtualPath` is the path relative to the current dir of the current program,
/// it's guaranteed that equivalent paths are interned to the same id
#[eterned::eterned]
pub struct VirtualPath {
    #[return_ref]
    _data: VirtualPathBuf,
}

impl Debug for VirtualPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data().fmt(f)
    }
}

impl ::salsa::DebugWithDb for VirtualPath {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        self.data().fmt(f)
    }
}

impl VirtualPath {
    #[inline(always)]
    pub fn data(self) -> &'static Path {
        self._data()
    }

    pub fn abs_path(self, db: &::salsa::Db) -> BaseVfsResult<PathBuf> {
        db.vfs_cache()
            .current_dir()
            .join(&self.data())
            .canonicalize()
            .map_err(|e| {
                p!(db.vfs_cache().current_dir().join(&self.data()), e);
                todo!()
            })
    }

    pub fn join(self, path: impl AsRef<Path>, db: &::salsa::Db) -> Self {
        VirtualPath::new(VirtualPathBuf(self.data().join(path)), db)
    }

    pub fn file(self, db: &::salsa::Db, durability: Durability) -> BaseVfsResult<File> {
        db.file_from_virtual_path(self, durability)
    }

    pub fn exists(self, db: &::salsa::Db, durability: Durability) -> BaseVfsResult<bool> {
        match self.file(db, durability)?.content(db) {
            FileContent::NotExists => Ok(false),
            FileContent::OnDisk(_) => Ok(true),
            FileContent::LiveDoc(_) => todo!(),
            FileContent::Directory(_) => Ok(true),
            FileContent::Err(_) => todo!(),
        }
    }

    pub fn text<'a>(
        self,
        db: &'a ::salsa::Db,
        durability: Durability,
    ) -> BaseVfsResult<Option<&'a str>> {
        let file = self.file(db, durability)?;
        Ok(file.text(db)?)
    }

    pub fn text_expected<'a>(
        self,
        db: &'a ::salsa::Db,
        durability: Durability,
    ) -> BaseVfsResult<&'a str> {
        let file = self.file(db, durability)?;
        file.text(db)?.ok_or(BaseVfsError::FileNotExists(self))
    }
}

impl VirtualPath {
    // todo: room for optimization when path is owned
    pub fn try_new(db: &::salsa::Db, path: impl AsRef<Path>) -> BaseVfsResult<Self> {
        Ok(Self::new(VirtualPathBuf::try_new(db, path.as_ref())?, db))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct VirtualPathBuf(PathBuf);

#[test]
fn test_absolute_path_debug() {
    let _db = DB::default();
    // let abs_path = VirtualPath::new(path);
}

impl VirtualPathBuf {
    pub fn try_new(db: &::salsa::Db, path: &Path) -> BaseVfsResult<Self> {
        let diff = |path: &Path| -> BaseVfsResult<_> {
            pathdiff::diff_paths(path, db.vfs_cache().current_dir())
                .ok_or(BaseVfsError::FailToDiffPaths)
        };
        let diff_path = if path.is_absolute() {
            diff(path)
        } else {
            diff(
                &std::path::absolute(&path).map_err(|e| BaseVfsError::FailToAbsolutize {
                    path: path.to_owned(),
                    error_message: e.to_string(),
                })?,
            )
        }?;
        Ok(VirtualPathBuf(diff_path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for VirtualPathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
