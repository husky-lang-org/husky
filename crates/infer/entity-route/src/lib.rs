mod builder;
mod query;
mod sheet;

pub use query::*;
pub use sheet::*;

use ast::*;
use check_utils::*;
use defn_head::*;
use entity_route::*;
use entity_route_query::{EntityRouteQueryGroup, EntityRouteResultArc};
use file::FilePtr;
use infer_decl::{CallDecl, DeclQueryGroup, MethodDecl, TyDecl};
use infer_error::*;
use print_utils::*;
use syntax_types::*;
use word::RootIdentifier;

pub trait InferEntityRoute {
    fn decl_db(&self) -> &dyn DeclQueryGroup;
    fn entity_route_sheet(&self) -> &EntityRouteSheet;
    fn expr_ty_result(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        msg_once!("deprecated");
        self.entity_route_sheet().expr_ty_result(expr_idx)
    }
    fn expr_ty_decl(&self, expr_idx: RawExprIdx) -> InferResultArc<TyDecl> {
        let ty = self.expr_ty_result(expr_idx)?;
        self.decl_db().ty_decl(ty)
    }

    fn call_route_result(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        self.entity_route_sheet().call_route(expr_idx)
    }

    fn call_decl(&self, expr_idx: RawExprIdx) -> InferResultArc<CallDecl> {
        self.decl_db().call_decl(self.call_route_result(expr_idx)?)
    }

    fn method_decl(&self, expr_idx: RawExprIdx) -> InferResultArc<MethodDecl> {
        self.decl_db()
            .method_decl(self.call_route_result(expr_idx)?)
    }
}

fn is_implicit_convertible(
    db: &dyn InferEntityRouteQueryGroup,
    src_ty: EntityRoutePtr,
    dst_ty: EntityRoutePtr,
) -> bool {
    if src_ty == dst_ty {
        return true;
    }
    match dst_ty {
        EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
            RootIdentifier::Void => false,
            RootIdentifier::I32 => {
                p!(src_ty, dst_ty);
                todo!()
            }
            RootIdentifier::F32 => todo!(),
            RootIdentifier::B32 => todo!(),
            RootIdentifier::B64 => todo!(),
            RootIdentifier::Bool => match src_ty {
                EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
                    RootIdentifier::I32
                    | RootIdentifier::F32
                    | RootIdentifier::B32
                    | RootIdentifier::B64
                    | RootIdentifier::Bool => true,
                    RootIdentifier::Void
                    | RootIdentifier::Vec
                    | RootIdentifier::Tuple
                    | RootIdentifier::Fp
                    | RootIdentifier::Fn
                    | RootIdentifier::FnMut
                    | RootIdentifier::FnOnce
                    | RootIdentifier::Array
                    | RootIdentifier::DatasetType
                    | RootIdentifier::Type => false,
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            RootIdentifier::True => todo!(),
            RootIdentifier::False => todo!(),
            RootIdentifier::Vec => todo!(),
            RootIdentifier::Tuple => todo!(),
            RootIdentifier::Debug => todo!(),
            RootIdentifier::Std => todo!(),
            RootIdentifier::Core => todo!(),
            RootIdentifier::Fp => todo!(),
            RootIdentifier::Fn => todo!(),
            RootIdentifier::FnMut => todo!(),
            RootIdentifier::FnOnce => todo!(),
            RootIdentifier::Array => todo!(),
            RootIdentifier::DatasetType => match src_ty {
                EntityRoutePtr::Root(RootIdentifier::DatasetType) => true,
                EntityRoutePtr::Custom(scope) => match scope.kind {
                    EntityRouteKind::Root {
                        ident: RootIdentifier::DatasetType,
                    } => true,
                    _ => false,
                },
                _ => false,
            },
            RootIdentifier::Type => todo!(),
            RootIdentifier::Datasets => panic!(),
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
        },
        EntityRoutePtr::Custom(_) => {
            p!(src_ty, dst_ty);
            todo!()
        }
        EntityRoutePtr::ThisType => todo!(),
    }
}
