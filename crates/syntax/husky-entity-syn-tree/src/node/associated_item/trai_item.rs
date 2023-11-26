use super::*;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitItemSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitItemSynNodePathData {
    parent_trai_syn_node_path: TraitSynNodePath,
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitItemPath>,
}

impl From<TraitItemSynNodePath> for ItemSynNodePath {
    fn from(path: TraitItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(path.into())
    }
}

impl TraitItemSynNodePath {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        parent_trai_syn_node_path: TraitSynNodePath,
        path: TraitItemPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssociatedItem(AssociatedItemSynNodeDataPathData::TraitItem(
                TraitItemSynNodePathData {
                    parent_trai_syn_node_path,
                    maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
                },
            )),
        ))
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TraitItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub(crate) fn syn_node(self, db: &::salsa::Db) -> TraitItemSynNodeData {
        trai_item_syn_node(db, self)
    }
}

impl HasSynNodePath for TraitItemPath {
    type SynNodePath = TraitItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitItemSynNodePath::new_inner(
            db,
            self.trai_path(db).syn_node_path(db),
            MaybeAmbiguousPath::from_path(self),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TraitItemSynNodeData {
    syn_node_path: TraitItemSynNodePath,
    ast_idx: AstIdx,
    ident: Ident,
    item_kind: TraitItemKind,
    visibility: Scope,
    is_generic: bool,
}

impl TraitItemSynNodeData {
    #[inline(always)]
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        trai_syn_node_path: TraitSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TraitItemSynNodePath, Self) {
        let trai_item_path = TraitItemPath::new(trai_syn_node_path.path(db), ident, item_kind, db);
        let syn_node_path =
            TraitItemSynNodePath::new(db, registry, trai_syn_node_path, trai_item_path);
        (
            syn_node_path,
            Self::new_inner(
                db,
                syn_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            ),
        )
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        todo!(); // self.syn_node_path(db).module_path(db)
    }
}

pub(crate) fn trai_item_syn_nodes(
    db: &::salsa::Db,
    trai_node_path: TraitSynNodePath,
) -> SmallVec<
    [(Ident, TraitItemSynNodePath, TraitItemSynNodeData);
        APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS],
> {
    let trai_node = trai_node_path.syn_node(db);
    let module_path = todo!(); // trai_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path);
    let DefnBlock::Trait { path: _, items } = trai_node.block(db) else {
        unreachable!()
    };
    let Some(items) = items else {
        return Default::default();
    };
    let mut registry = ItemSynNodePathRegistry::default();
    items
        .ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Identifiable {
                    visibility_expr,
                    item_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitItem(item_kind),
                    } = item_kind
                    else {
                        unreachable!()
                    };
                    let (syn_node_path, node) = TraitItemSynNodeData::new(
                        db,
                        &mut registry,
                        trai_node_path,
                        ast_idx,
                        ident_token.ident(),
                        *item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), syn_node_path, node))
                }
                Ast::Err { .. } => None,
                _ => unreachable!(),
            }
        })
        .collect()
}

pub(crate) fn trai_item_syn_node(
    db: &::salsa::Db,
    syn_node_path: TraitItemSynNodePath,
) -> TraitItemSynNodeData {
    syn_node_path
        .parent_trai_syn_node_path(db)
        .associated_items(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path1 == syn_node_path).then_some(node))
        .expect("some")
}

pub(crate) const APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS: usize = 4;
