use crate::EntityDefn;
use crate::*;
use atom::AtomContext;
use dev_utils::DevSource;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefn, MethodStaticDefnVariant, StaticTraitImplDefn};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDefn {}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDefn {
    pub trai: EntityRoutePtr,
    pub member_impls: Vec<Arc<EntityDefn>>,
    pub dev_src: DevSource,
}

impl TraitImplDefn {
    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        static_trait_impl: &StaticTraitImplDefn,
    ) -> Arc<Self> {
        let trai = symbol_context
            .parse_entity_route(static_trait_impl.trai)
            .unwrap();
        Arc::new(Self {
            trai,
            member_impls: static_trait_impl.member_impls.map(|static_trait_impl| {
                EntityDefn::trait_member_impl_from_static(symbol_context, trai, static_trait_impl)
                // match static_trait_impl {
                //     StaticTraitMemberImplDecl::Type { name, route } => {
                //     }
                // }
            }),
            dev_src: static_trait_impl.dev_src.into(),
        })
    }

    pub fn member_impl(&self, ident: CustomIdentifier) -> &Arc<EntityDefn> {
        self.member_impls
            .iter()
            .find(|member_impl| member_impl.ident.custom() == ident)
            .unwrap()
    }
}

impl EntityDefn {
    pub fn trait_member_impl_from_static(
        context: &mut dyn AtomContext,
        trai: EntityRoutePtr,
        static_trait_impl: &EntityStaticDefn,
    ) -> Arc<Self> {
        let variant =
            EntityDefnVariant::trait_member_impl_from_static(context, trai, static_trait_impl);
        let ident = context
            .entity_syntax_db()
            .intern_word(static_trait_impl.name)
            .ident();
        Self::new(
            ident,
            variant,
            context.opt_this_ty().unwrap(),
            context
                .entity_syntax_db()
                .intern_file(static_trait_impl.dev_src.file.into()),
            static_trait_impl.dev_src.into(),
        )
    }
}

impl EntityDefnVariant {
    pub fn trait_member_impl_from_static(
        context: &mut dyn AtomContext,
        trai: EntityRoutePtr,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => {
                Self::TraitAssociatedTypeImpl {
                    trai,
                    ty: context.parse_entity_route(ty).unwrap(),
                }
            }
            EntityStaticDefnVariant::Method {
                this_liason: this_contract,
                parameters: inputs,
                output_ty,
                output_liason,
                spatial_parameters: generic_parameters,
                ref kind,
            } => {
                let method_variant = match kind {
                    MethodStaticDefnVariant::TypeMethod { source } => todo!(),
                    MethodStaticDefnVariant::TraitMethod { opt_default_source } => {
                        MethodDefnVariant::TraitMethod {
                            trai,
                            opt_default_source: opt_default_source
                                .clone()
                                .map(|source| CallFormSource::Static(source)),
                        }
                    }
                    MethodStaticDefnVariant::TraitMethodImpl { opt_source } => {
                        MethodDefnVariant::TraitMethodImpl {
                            trai,
                            opt_source: opt_source
                                .clone()
                                .map(|source| CallFormSource::Static(source)),
                        }
                    }
                };
                Self::Method {
                    parameters: Arc::new(inputs.map(|input_placeholder| {
                        context.input_placeholder_from_static(input_placeholder)
                    })),
                    output_ty: RangedEntityRoute {
                        route: context.parse_entity_route(output_ty).unwrap(),
                        range: Default::default(),
                    },
                    this_contract,
                    output_liason,
                    method_variant,
                    generic_parameters: generic_parameters.map(|generic_placeholder| {
                        SpatialParameter::from_static(
                            context.entity_syntax_db(),
                            generic_placeholder,
                        )
                    }),
                }
            }
            _ => panic!(),
        }
    }
}
