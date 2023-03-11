use husky_entity_taxonomy::AssociatedItemKind;
use husky_print_utils::p;
use husky_word::IdentPairMap;

use crate::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct AssociatedItem {
    #[id]
    pub id: AssociatedItemId,
    pub path: Option<AssociatedItemPath>,
    pub impl_block: ImplBlock,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub associated_item_kind: AssociatedItemKind,
    pub accessibility: Accessibility,
    pub is_generic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct AssociatedItemId {
    impl_id: ImplBlockId,
    ident: Ident,
}

impl AssociatedItemId {
    pub fn module_path(self) -> ModulePath {
        self.impl_id.module_path()
    }
}

impl AssociatedItem {
    pub fn new_impl_associated_item(
        db: &dyn EntityTreeDb,
        impl_block: ImplBlock,
        ast_idx: AstIdx,
        ident: Ident,
        associated_item_kind: AssociatedItemKind,
        accessibility: Accessibility,
        is_generic: bool,
    ) -> Self {
        let id = AssociatedItemId {
            impl_id: impl_block.id(db),
            ident,
        };
        let path: Option<AssociatedItemPath> = match associated_item_kind {
            AssociatedItemKind::TraitItem(_) => todo!(),
            AssociatedItemKind::TypeItem(ty_item_kind) => match impl_block.kind(db) {
                ImplBlockKind::Type { ty } => {
                    Some(TypeItemPath::new(db, ty, ident, ty_item_kind).into())
                }
                ImplBlockKind::Err => None,
                _ => unreachable!(),
            },
            AssociatedItemKind::TypeAsTraitItem(ty_as_trai_item_kind) => {
                match impl_block.kind(db) {
                    ImplBlockKind::TypeAsTrait { ty, trai } => Some(
                        TypeAsTraitItemPath::new(db, ty, trai, ident, ty_as_trai_item_kind).into(),
                    ),
                    ImplBlockKind::Err => None,
                    _ => {
                        p!(impl_block.kind(db));
                        unreachable!()
                    }
                }
            }
        };
        Self::new(
            db,
            id,
            path,
            impl_block,
            ast_idx,
            ident,
            associated_item_kind,
            accessibility,
            is_generic,
        )
    }

    pub fn module_path(&self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).impl_id.module_path()
    }
}

impl AsVecMapEntry for AssociatedItem {
    type K = AssociatedItemPath;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        todo!()
    }

    fn key_ref(&self) -> &Self::K {
        todo!()
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_impl_block_associated_items(
    db: &dyn EntityTreeDb,
    impl_block: TypeImplBlock,
) -> IdentPairMap<AssociatedItem> {
    todo!()
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_as_trai_impl_block_associated_items(
    db: &dyn EntityTreeDb,
    impl_block: TypeAsTraitImplBlock,
) -> IdentPairMap<AssociatedItem> {
    todo!()
}

pub(crate) fn impl_block_associated_items_aux(
    db: &dyn EntityTreeDb,
    impl_block: ImplBlock,
    module_path: ModulePath,
    body: AstIdxRange,
) -> IdentPairMap<AssociatedItem> {
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    body.into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Defn {
                    accessibility,
                    entity_kind,

                    ident_token,
                    is_generic,
                    ..
                } => {
                    let associated_item_kind = match entity_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind,
                        } => *associated_item_kind,
                        _ => unreachable!(),
                    };
                    Some((
                        ident_token.ident(),
                        AssociatedItem::new_impl_associated_item(
                            db,
                            impl_block,
                            ast_idx,
                            ident_token.ident(),
                            associated_item_kind,
                            *accessibility,
                            *is_generic,
                        ),
                    ))
                }
                Ast::Err { .. } => None,
                _ => {
                    p!(impl_block.debug(db));
                    p!(ast);
                    unreachable!()
                }
            }
        })
        .collect()
}
