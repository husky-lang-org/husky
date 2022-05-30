use super::*;

impl EntityDefnVariant {
    pub(crate) fn type_field_from_ast(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        field_defn_head: &FieldDefnHead,
        children: Option<AstIter>,
    ) -> SemanticResult<Self> {
        let variant = match field_defn_head.kind {
            FieldKind::StructOriginal => FieldDefnVariant::StructOriginal,
            FieldKind::StructDerivedLazy { paradigm } => FieldDefnVariant::StructDerived {
                defn_repr: Arc::new(match paradigm {
                    Paradigm::LazyFunctional => DefinitionRepr::LazyBlock { stmts: todo!() },
                    Paradigm::EagerFunctional => DefinitionRepr::FuncBlock {
                        stmts: semantics_eager::parse_func_stmts(
                            &[],
                            db,
                            arena,
                            children.unwrap(),
                            file,
                        )?,
                    },
                    Paradigm::EagerProcedural => todo!(),
                }),
            },
            FieldKind::RecordOriginal => FieldDefnVariant::RecordOriginal,
            FieldKind::RecordDerived => FieldDefnVariant::RecordDerived {
                defn_repr: Arc::new(DefinitionRepr::LazyBlock {
                    stmts: semantics_lazy::parse_lazy_stmts(
                        &[],
                        db,
                        arena,
                        children.unwrap(),
                        file,
                    )?,
                }),
            },
            _ => todo!(),
        };
        Ok(Self::TypeField {
            ty: field_defn_head.ty.route,
            contract: field_defn_head.liason,
            field_variant: variant,
        })
    }

    pub(crate) fn collect_original_fields(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
        ty_route: EntityRoutePtr,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            match ast.variant {
                AstKind::FieldDefnHead { ref head, .. } => {
                    match head.kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::RecordOriginal => (),
                        _ => break,
                    }
                    members.insert_new(EntityDefn::new(
                        head.ident.ident.into(),
                        EntityDefnVariant::type_field_from_ast(
                            db,
                            arena,
                            file,
                            head,
                            child.opt_children.clone(),
                        )?,
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: head.ident.ident,
                            },
                            generic_arguments: vec![],
                        }),
                        file,
                        ast.range,
                    ));
                    children.next();
                }
                _ => break,
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldDefnVariant {
    StructOriginal,
    RecordOriginal,
    StructDerived { defn_repr: Arc<DefinitionRepr> },
    RecordDerived { defn_repr: Arc<DefinitionRepr> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionRepr {
    EagerExpr {},
    LazyBlock { stmts: Arc<Vec<Arc<LazyStmt>>> },
    FuncBlock { stmts: Arc<Vec<Arc<FuncStmt>>> },
    ProcBlock { stmts: Arc<Vec<Arc<ProcStmt>>> },
}
