use crate::*;
use atom::{
    symbol::{Symbol, SymbolContextKind, SymbolKind},
    *,
};
use entity_route::*;
use fold::LocalStack;
use map_collect::MapCollect;
use word::IdentDict;

impl<'a> dyn DeclQueryGroup + 'a {
    // pub fn parse_entity(
    //     &self,
    //     text: &str,
    //     symbol_context: &SymbolContext,
    // ) -> AtomResult<EntityRoutePtr> {
    //     parse_entity(symbol_context, &self.tokenize(text))
    // }

    pub fn generic_parameters_from_static(
        &self,
        static_generic_parameters: &[StaticGenericPlaceholder],
    ) -> IdentDict<GenericParameter> {
        static_generic_parameters.map(|static_generic_placeholder| GenericParameter {
            ident: self.intern_word(static_generic_placeholder.name).custom(),
            variant: GenericPlaceholderVariant::Type { traits: vec![] },
        })
    }

    pub fn generic_arguments_from_generic_parameters(
        &self,
        generic_parameters: &[GenericParameter],
    ) -> Vec<GenericArgument> {
        generic_parameters.map(|generic_placeholder| {
            GenericArgument::EntityRoute(self.intern_entity_route(EntityRoute {
                kind: EntityRouteKind::Generic {
                    ident: generic_placeholder.ident,
                    entity_kind: generic_placeholder.entity_kind(),
                },
                generic_arguments: vec![],
            }))
        })
    }

    pub fn symbols_from_generic_parameters(
        &self,
        generic_parameters: &[GenericParameter],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for generic_placeholder in generic_parameters.iter() {
            symbols.push(Symbol {
                ident: generic_placeholder.ident,
                kind: SymbolKind::EntityRoute(self.intern_entity_route(EntityRoute {
                    kind: EntityRouteKind::Generic {
                        ident: generic_placeholder.ident,
                        entity_kind: generic_placeholder.entity_kind(),
                    },
                    generic_arguments: vec![],
                })),
            })
        }
        symbols
    }
}
