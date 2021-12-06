use std::{fmt, iter::FromIterator, sync::Arc};

use ide_db::base_db::{
    salsa::debug::{DebugQueryTable, TableEntry},
    CrateId, FileID, FileTextQuery, SourceDatabase, SourceRootId,
};
use ide_db::{
    symbol_index::{LibrarySymbolsQuery, SymbolIndex},
    IdeDatabase,
};
use itertools::Itertools;
use profile::{memory_usage, Bytes};
use rustc_hash::FxHashMap;
use std::env;
use stdx::format_to;
use syntax::{ast, ParseResult, SyntaxNode};

fn syntax_tree_stats(db: &IdeDatabase) -> SyntaxTreeStats {
    todo!()
}

// Feature: Status
//
// Shows internal statistic about memory usage of husky-lang-server.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Status**
// |===
// image::https://user-images.githubusercontent.com/48062697/113065584-05f34500-91b1-11eb-98cc-5c196f76be7f.gif[]
pub(crate) fn status(db: &IdeDatabase, file_id: Option<FileID>) -> String {
    let mut buf = String::new();
    format_to!(buf, "{}\n", FileTextQuery.in_db(db).entries::<FilesStats>());
    format_to!(
        buf,
        "{}\n",
        LibrarySymbolsQuery
            .in_db(db)
            .entries::<LibrarySymbolsStats>()
    );
    format_to!(buf, "{}\n", syntax_tree_stats(db));
    format_to!(buf, "{} in total\n", memory_usage());
    if env::var("RA_COUNT").is_ok() {
        format_to!(buf, "\nCounts:\n{}", profile::countme::get_all());
    }

    if let Some(file_id) = file_id {
        format_to!(buf, "\nFile info:\n");
        let krate = crate::parent_module::crate_for(db, file_id).pop();
        match krate {
            Some(krate) => {
                let crate_graph = db.crate_graph();
                let display_crate = |krate: CrateId| match &crate_graph[krate].display_name {
                    Some(it) => format!("{}({:?})", it, krate),
                    None => format!("{:?}", krate),
                };
                format_to!(buf, "Crate: {}\n", display_crate(krate));
                let deps = crate_graph[krate]
                    .dependencies
                    .iter()
                    .map(|dep| format!("{}={:?}", dep.name, dep.crate_id))
                    .format(", ");
                format_to!(buf, "Dependencies: {}\n", deps);
            }
            None => format_to!(buf, "Does not belong to any crate"),
        }
    }

    buf.trim().to_string()
}

#[derive(Default)]
struct FilesStats {
    total: usize,
    size: Bytes,
}

impl fmt::Display for FilesStats {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} of files", self.size)
    }
}

impl FromIterator<TableEntry<FileID, Arc<String>>> for FilesStats {
    fn from_iter<T>(iter: T) -> FilesStats
    where
        T: IntoIterator<Item = TableEntry<FileID, Arc<String>>>,
    {
        let mut res = FilesStats::default();
        for entry in iter {
            res.total += 1;
            res.size += entry.value.unwrap().len();
        }
        res
    }
}

#[derive(Default)]
pub(crate) struct SyntaxTreeStats {
    total: usize,
    pub(crate) retained: usize,
}

impl fmt::Display for SyntaxTreeStats {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} trees, {} preserved", self.total, self.retained)
    }
}

impl FromIterator<TableEntry<FileID, ParseResult<ast::SingleFileParseTree>>> for SyntaxTreeStats {
    fn from_iter<T>(iter: T) -> SyntaxTreeStats
    where
        T: IntoIterator<Item = TableEntry<FileID, ParseResult<ast::SingleFileParseTree>>>,
    {
        let mut res = SyntaxTreeStats::default();
        for entry in iter {
            res.total += 1;
            res.retained += entry.value.is_some() as usize;
        }
        res
    }
}

#[derive(Default)]
struct LibrarySymbolsStats {
    total: usize,
    size: Bytes,
}

impl fmt::Display for LibrarySymbolsStats {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} of index symbols ({})", self.size, self.total)
    }
}

impl FromIterator<TableEntry<(), Arc<FxHashMap<SourceRootId, SymbolIndex>>>>
    for LibrarySymbolsStats
{
    fn from_iter<T>(iter: T) -> LibrarySymbolsStats
    where
        T: IntoIterator<Item = TableEntry<(), Arc<FxHashMap<SourceRootId, SymbolIndex>>>>,
    {
        let mut res = LibrarySymbolsStats::default();
        for entry in iter {
            let value = entry.value.unwrap();
            for symbols in value.values() {
                res.total += symbols.len();
                res.size += symbols.memory_size();
            }
        }
        res
    }
}
