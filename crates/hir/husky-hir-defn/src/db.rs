use crate::*;
use husky_hir_decl::db::HirDeclDb;

pub trait HirDefnDb: salsa::DbWithJar<HirDefnJar> + HirDeclDb {}

impl HirDefnDb for Db where Db: salsa::DbWithJar<HirDefnJar> + HirDeclDb {}

#[salsa::jar(db = HirDefnDb)]
pub struct HirDefnJar(
    // defn
    // - type
    ty_hir_defn,
    EnumHirDefn,
    crate::defn::enum_hir_defn_dependencies,
    crate::defn::enum_hir_defn_version_stamp,
    UnitStructHirDefn,
    crate::defn::unit_struct_hir_defn_dependencies,
    crate::defn::unit_struct_hir_defn_version_stamp,
    TupleStructHirDefn,
    crate::defn::tuple_struct_hir_defn_dependencies,
    crate::defn::tuple_struct_hir_defn_version_stamp,
    PropsStructHirDefn,
    crate::defn::props_struct_hir_defn_dependencies,
    crate::defn::props_struct_hir_defn_version_stamp,
    ExternHirDefn,
    crate::defn::extern_hir_defn_dependencies,
    crate::defn::extern_hir_defn_version_stamp,
    UnionHirDefn,
    crate::defn::union_hir_defn_dependencies,
    crate::defn::union_hir_defn_version_stamp,
    // - fugitive
    // fugitive_hir_defn,
    ValHirDefn,
    crate::defn::val_hir_defn_dependencies,
    crate::defn::val_hir_defn_version_stamp,
    FunctionFnHirDefn,
    crate::defn::function_fn_hir_defn_dependencies,
    crate::defn::function_fn_hir_defn_version_stamp,
    FunctionGnHirDefn,
    crate::defn::function_gn_hir_defn_dependencies,
    crate::defn::function_gn_hir_defn_version_stamp,
    // - morphism_defn,
    TypeAliasHirDefn,
    crate::defn::ty_alias_hir_defn_dependencies,
    crate::defn::ty_alias_hir_defn_version_stamp,
    // - type_alias_defn,
    // - trait
    TraitHirDefn,
    trai_hir_defn,
    crate::defn::trai_hir_defn_dependencies,
    crate::defn::trai_hir_defn_version_stamp,
    // - enum variant,
    EnumUnitVariantHirDefn,
    crate::defn::enum_unit_variant_hir_defn_dependencies,
    crate::defn::enum_unit_variant_hir_defn_version_stamp,
    EnumTupleVariantHirDefn,
    crate::defn::enum_tuple_variant_hir_defn_dependencies,
    crate::defn::enum_tuple_variant_hir_defn_version_stamp,
    EnumPropsVariantHirDefn,
    crate::defn::enum_props_variant_hir_defn_dependencies,
    crate::defn::enum_props_variant_hir_defn_version_stamp,
    // - type item
    // ty_item_hir_defn,
    TypeAssociatedFnHirDefn,
    crate::defn::ty_associated_fn_hir_defn_dependencies,
    crate::defn::ty_associated_fn_hir_defn_version_stamp,
    TypeMethodFnHirDefn,
    crate::defn::ty_method_fn_hir_defn_dependencies,
    crate::defn::ty_method_fn_hir_defn_version_stamp,
    TypeAssociatedTypeHirDefn,
    crate::defn::ty_associated_ty_hir_defn_dependencies,
    crate::defn::ty_associated_ty_hir_defn_version_stamp,
    TypeAssociatedValHirDefn,
    crate::defn::ty_associated_val_hir_defn_dependencies,
    crate::defn::ty_associated_val_hir_defn_version_stamp,
    TypeMemoizedFieldHirDefn,
    crate::defn::ty_memoized_field_hir_defn_dependencies,
    crate::defn::ty_memoized_field_hir_defn_version_stamp,
    // - trait item
    // trai_item_hir_defn,
    TraitAssociatedFnHirDefn,
    crate::defn::trai_associated_fn_hir_defn_dependencies,
    crate::defn::trai_associated_fn_hir_defn_version_stamp,
    TraitMethodFnHirDefn,
    crate::defn::trai_method_fn_hir_defn_dependencies,
    crate::defn::trai_method_fn_hir_defn_version_stamp,
    TraitAssociatedTypeHirDefn,
    crate::defn::trai_associated_ty_hir_defn_dependencies,
    crate::defn::trai_associated_ty_hir_defn_version_stamp,
    TraitAssociatedValHirDefn,
    crate::defn::trai_associated_val_hir_defn_dependencies,
    crate::defn::trai_associated_val_hir_defn_version_stamp,
    // - trait for type item
    // trai_for_ty_item_hir_defn,
    TraitForTypeAssociatedFnHirDefn,
    crate::defn::trai_for_ty_associated_fn_hir_defn_dependencies,
    crate::defn::trai_for_ty_associated_fn_hir_defn_version_stamp,
    TraitForTypeMethodFnHirDefn,
    crate::defn::trai_for_ty_method_fn_hir_defn_dependencies,
    crate::defn::trai_for_ty_method_fn_hir_defn_version_stamp,
    TraitForTypeAssociatedTypeHirDefn,
    crate::defn::trai_for_ty_associated_ty_hir_defn_dependencies,
    crate::defn::trai_for_ty_associated_ty_hir_defn_version_stamp,
    TraitForTypeAssociatedValHirDefn,
    crate::defn::trai_for_ty_associated_val_hir_defn_dependencies,
    crate::defn::trai_for_ty_associated_val_hir_defn_version_stamp,
    // - impl block
    crate::defn::ty_impl_block_dependencies,
    crate::defn::ty_impl_block_version_stamp,
    crate::defn::trai_for_ty_impl_block_dependencies,
    crate::defn::trai_for_ty_impl_block_version_stamp,
    // dependencies
    crate::dependencies::HirDefnDependencies,
    // version stamp
    crate::version_stamp::HirDefnVersionStamp,
);
