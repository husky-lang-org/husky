use word::ContextualIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureDecl {
    pub ty: EntityRoutePtr,
}

pub(crate) fn feature_decl(
    db: &dyn DeclQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<FeatureDecl> {
    let source = db.entity_source(scope)?;
    match source {
        EntitySource::StaticModuleItem(data) => todo!(),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::FeatureDecl { ident, ty } => Ok(Arc::new(FeatureDecl { ty: ty.route })),
                _ => todo!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { main } => Ok(Arc::new(FeatureDecl {
            ty: db.global_input_ty(main)?,
        })),
        EntitySource::StaticTypeMember => todo!(),
        EntitySource::StaticTypeAsTraitMember => todo!(),
    }
}
