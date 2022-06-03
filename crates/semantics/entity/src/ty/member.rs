mod field;
mod method;

pub use field::*;
pub use method::*;
use word::Paradigm;

use super::*;

pub fn collect_all_members(
    type_members: &[Arc<EntityDefn>],
    trait_impls: &[Arc<TraitImplDefn>],
) -> Avec<EntityDefn> {
    let mut members = type_members.to_vec();
    for trait_impl in trait_impls {
        members.extend(
            trait_impl
                .member_impls
                .iter()
                .map(|member_impl| member_impl.clone()),
        );
    }
    Arc::new(members)
}

pub fn member_defn(db: &dyn EntityDefnQueryGroup, member_route: EntityRoutePtr) -> Arc<EntityDefn> {
    let ty = member_route.parent();
    let ty_defn = db.entity_defn(ty).unwrap();
    let member_idx = db.member_idx(member_route);
    match ty_defn.variant {
        EntityDefnVariant::Type { ref members, .. } => members[member_idx.0 as usize].clone(),
        _ => panic!(),
    }
}

impl EntityDefnVariant {
    pub(crate) fn collect_other_ty_members(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            let (ident, variant): (CustomIdentifier, _) = match ast.variant {
                AstVariant::TypeDefnHead {
                    ident,
                    kind,
                    ref generic_parameters,
                } => todo!(),
                AstVariant::MainDefn => todo!(),
                AstVariant::CallFormDefnHead(ref head) => match head.opt_this_contract {
                    Some(this_contract) => {
                        let method_source = match head.paradigm {
                            Paradigm::EagerProcedural => todo!(),
                            Paradigm::EagerFunctional => {
                                let stmts = semantics_eager::parse_func_stmts(
                                    db,
                                    arena,
                                    child.opt_children.clone().unwrap(),
                                    file,
                                )?;
                                MethodSource::Func { stmts }
                            }
                            Paradigm::LazyFunctional => todo!(),
                        };
                        let method_variant = MethodDefnVariant::TypeMethod {
                            ty: ty_route,
                            method_source,
                        };
                        (
                            head.ident.ident,
                            EntityDefnVariant::Method {
                                parameters: head.parameters.clone(),
                                output_ty: head.output_ty,
                                this_contract,
                                method_variant,
                                output_liason: OutputLiason::Transfer,
                                generic_parameters: head.generic_parameters.clone(),
                            },
                        )
                    }
                    None => (
                        head.ident.ident,
                        EntityDefnVariant::function(
                            db,
                            head,
                            child.opt_children.clone().unwrap(),
                            arena,
                            file,
                        )?,
                    ),
                },
                AstVariant::PatternDefnHead => todo!(),
                AstVariant::FeatureDecl { ident, ty } => todo!(),
                AstVariant::Use { .. } => todo!(),
                AstVariant::FieldDefnHead { ranged_ident, .. } => (
                    ranged_ident.ident,
                    EntityDefnVariant::ty_field_from_ast(
                        db,
                        arena,
                        file,
                        ty_route,
                        ast,
                        child.opt_children.clone(),
                    )?,
                ),
                AstVariant::DatasetConfigDefnHead => todo!(),
                AstVariant::Stmt(_) => todo!(),
                AstVariant::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
                AstVariant::Submodule { ident, source_file } => todo!(),
                AstVariant::Visual => break,
            };
            children.next();
            members.insert_new(EntityDefn::new(
                ident.into(),
                variant,
                db.make_subroute(ty_route, ident, Vec::new()),
                file,
                ast.range,
            ))
        }
        Ok(())
    }
}
