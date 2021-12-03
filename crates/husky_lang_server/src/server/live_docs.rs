//! In-memory document information.

use std::mem;

use rustc_hash::FxHashMap;
use vfs::VfsPath;

/// Holds the set of in-memory documents.
///
/// For these document, there true contents is maintained by the client. It
/// might be different from what's on disk.
#[derive(Default, Clone)]
pub(crate) struct LiveDocs {
    live_docs: FxHashMap<VfsPath, DocumentData>,
    added_or_removed: bool,
}

impl LiveDocs {
    pub(crate) fn contains(&self, path: &VfsPath) -> bool {
        self.live_docs.contains_key(path)
    }
    pub(crate) fn insert(&mut self, path: VfsPath, data: DocumentData) -> Result<(), ()> {
        self.added_or_removed = true;
        match self.live_docs.insert(path, data) {
            Some(_) => Err(()),
            None => Ok(()),
        }
    }
    pub(crate) fn remove(&mut self, path: &VfsPath) -> Result<(), ()> {
        self.added_or_removed = true;
        match self.live_docs.remove(path) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }
    pub(crate) fn get(&self, path: &VfsPath) -> Option<&DocumentData> {
        self.live_docs.get(path)
    }
    pub(crate) fn get_mut(&mut self, path: &VfsPath) -> Option<&mut DocumentData> {
        // NB: don't set `self.added_or_removed` here, as that purposefully only
        // tracks changes to the key set.
        self.live_docs.get_mut(path)
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &VfsPath> {
        self.live_docs.keys()
    }
    pub(crate) fn take_changes(&mut self) -> bool {
        mem::replace(&mut self.added_or_removed, false)
    }
}

/// Information about a document that the Language Client
/// knows about.
/// Its lifetime is driven by the textDocument/didOpen and textDocument/didClose
/// client notifications.
#[derive(Debug, Clone)]
pub(crate) struct DocumentData {
    pub(crate) version: i32,
}

impl DocumentData {
    pub(crate) fn new(version: i32) -> Self {
        DocumentData { version }
    }
}
