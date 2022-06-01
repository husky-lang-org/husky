use entity_route::*;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_entity_route(&mut self, scope: EntityRoutePtr) {
        match scope.kind {
            EntityRouteKind::Root { ident } => self.result += &ident,
            EntityRouteKind::Package { .. } => self.write("crate"),
            EntityRouteKind::Child { parent, ident } => {
                self.gen_entity_route(parent);
                self.write("::");
                self.write(&ident)
            }
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TypeAsTraitMember {
                ty: parent,
                trai,
                ident,
            } => todo!(),
        }
        if scope.spatial_arguments.len() > 0 {
            self.write("<");
            for i in 0..scope.spatial_arguments.len() {
                if i > 0 {
                    self.write(", ")
                }
                match scope.spatial_arguments[i] {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(entity_route) => {
                        self.gen_entity_route(entity_route)
                    }
                }
            }
        }
    }
}
