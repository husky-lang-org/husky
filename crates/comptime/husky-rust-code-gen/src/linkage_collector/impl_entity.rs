use super::*;
use husky_entity_semantics::{
    CallFormSource, DefinitionRepr, EntityDefn, EntityDefnVariant, EnumVariantDefnVariant,
    FieldDefnVariant, MethodDefnKind,
};

impl<'a> LinkageCollector<'a> {
    pub(crate) fn collect_from_entity_defn(&mut self, defn: &EntityDefn) {
        match defn.variant {
            EntityDefnVariant::Module {
                ref module_items,
                ref opt_main_defn,
            } => {
                if let Some(main_defn) = opt_main_defn {
                    self.collect_from_feature_repr(None, &main_defn.defn_repr)
                }
                module_items
                    .iter()
                    .for_each(|item| self.collect_from_entity_defn(item))
            }
            EntityDefnVariant::Feature { ref defn_repr } => {
                self.collect_from_feature_repr(Some(defn.base_route), defn_repr)
            }
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => self.collect_from_call_form_source(source),
            EntityDefnVariant::Method {
                ref spatial_parameters,
                this_liason,
                ref parameters,
                output_ty,
                output_liason,
                method_defn_kind,
                ref opt_source,
            } => {
                self.insert(defn.base_route);
                self.insert(defn.base_route.parent());
                self.collect_from_parameters(parameters);
                self.insert(output_ty.route);
                if let Some(source) = opt_source {
                    self.collect_from_call_form_source(source)
                }
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.insert(defn.base_route);
                self.collect_from_parameters(parameters);
                self.insert(output.route);
                self.collect_from_func_stmts(stmts)
            }
            EntityDefnVariant::Proc {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.insert(defn.base_route);
                self.collect_from_parameters(parameters);
                self.insert(output.route);
                self.collect_from_proc_stmts(stmts)
            }
            EntityDefnVariant::Ty {
                ref spatial_parameters,
                ref ty_members,
                ref variants,
                ty_kind: kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                ..
            } => {
                if opt_type_call.is_some() {
                    self.insert(defn.base_route)
                }
                let entity_route_menu = self.db.entity_route_menu();
                for member in members.iter() {
                    match member.variant {
                        EntityDefnVariant::TyField {
                            field_ty,
                            ref field_variant,
                            liason,
                            opt_linkage,
                        } => self.insert(field_ty),
                        EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {
                            if defn.base_route == entity_route_menu.clone_trait {
                                ()
                            } else {
                                self.insert(defn.base_route)
                            }
                        }
                        EntityDefnVariant::Method {
                            method_defn_kind, ..
                        } => match method_defn_kind {
                            MethodDefnKind::TypeMethod { .. } => self.insert(defn.base_route),
                            MethodDefnKind::TraitMethod { trai } => self.insert(defn.base_route),
                            MethodDefnKind::TraitMethodImpl { trai } => {
                                self.insert(defn.base_route)
                            }
                        },
                        _ => self.insert(member.base_route),
                    }
                }
            }
            EntityDefnVariant::Trait {
                ref spatial_parameters,
                ref members,
            } => {
                msg_once!("ad hoc ignore")
            }
            EntityDefnVariant::EnumVariant {
                ref enum_variant_defn_variant,
            } => match enum_variant_defn_variant {
                EnumVariantDefnVariant::Constant => todo!(),
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                ref field_variant, ..
            } => match field_variant {
                FieldDefnVariant::StructOriginal => todo!(),
                FieldDefnVariant::StructDefault { default } => todo!(),
                FieldDefnVariant::StructDerivedEager { derivation } => todo!(),
                FieldDefnVariant::StructDerivedLazy { defn_repr } => {
                    self.collect_from_feature_repr(None, defn_repr)
                }
                FieldDefnVariant::RecordOriginal => todo!(),
                FieldDefnVariant::RecordDerived { defn_repr } => todo!(),
            },
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {
                todo!()
            }
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => {
                todo!()
            }
            EntityDefnVariant::Input { .. } => todo!(),
            EntityDefnVariant::Any => (),
        }
    }

    fn collect_from_parameters(&mut self, parameters: &[Parameter]) {
        for parameter in parameters {
            self.insert(parameter.ty())
        }
    }

    fn collect_from_feature_repr(
        &mut self,
        opt_feature_route: Option<EntityRoutePtr>,
        feature_repr: &DefinitionRepr,
    ) {
        match feature_repr {
            DefinitionRepr::LazyExpr { expr } => todo!(),
            DefinitionRepr::LazyBlock { stmts, ty } => {
                opt_feature_route.map(|feature_route| self.insert(feature_route));
                self.insert(ty.route);
                self.collect_from_lazy_stmts(stmts)
            }
            DefinitionRepr::FuncBlock {
                route,
                file,
                range,
                stmts,
                return_ty: output_ty,
            } => {
                opt_feature_route.map(|feature_route| self.insert(feature_route));
                self.insert(output_ty.route);
                self.collect_from_func_stmts(stmts)
            }
            DefinitionRepr::ProcBlock {
                file,
                range,
                stmts,
                return_ty,
                ..
            } => {
                opt_feature_route.map(|feature_route| self.insert(feature_route));
                self.insert(return_ty.route);
                self.collect_from_proc_stmts(stmts)
            }
        }
    }

    fn collect_from_call_form_source(&mut self, source: &CallFormSource) {
        match source {
            CallFormSource::Func { stmts } => self.collect_from_func_stmts(stmts),
            CallFormSource::Proc { stmts } => self.collect_from_proc_stmts(stmts),
            CallFormSource::Lazy { stmts } => todo!(),
            CallFormSource::Static(_) => (),
        }
    }
}
