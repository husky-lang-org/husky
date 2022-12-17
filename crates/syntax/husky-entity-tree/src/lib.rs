#![feature(trait_upcasting)]
mod absolute;
mod alias;
mod ancestry;
mod builder;
mod db;
mod error;
mod implementation;
mod node;
mod submodule;
mod taxonomy;
#[cfg(test)]
mod tests;
mod visibility;

pub use absolute::*;
pub use alias::*;
pub use db::EntityTreeDb;
pub use error::*;

use ancestry::*;
use builder::*;
use error::EntityTreeError;
use husky_ast::AstIdx;
use husky_entity_card::EntityCard;
use husky_entity_path::*;
use husky_package_path::*;
use husky_vfs::VfsResult;
use idx_arena::{Arena, ArenaIdxRange};
use implementation::ImplementationMap;
use node::*;
use submodule::*;
use taxonomy::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar(
    absolute_entity_path,
    entity_tree_sheet,
    submodules,
    module_level_use_alls,
    module_level_use_ones,
    entity_node,
    closest_module_in_apparent_ancestry,
    taxonomy::entity_class,
    visibility::visibility,
);

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeSheet {
    arena: EntityNodeArena,
    top_level_nodes: EntityNodeIdxRange,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn entity_tree_sheet(db: &dyn EntityTreeDb, module: EntityPath) -> VfsResult<EntityTreeSheet> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    EntityTreeBuilder::new(db, ast_sheet).build()
}
