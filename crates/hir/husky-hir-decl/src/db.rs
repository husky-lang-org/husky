use crate::*;

#[salsa::jar(db = HirDeclDb)]
pub struct HirDeclJar(
    submodule_hir_decl,
    SubmoduleHirDecl,
    // associated_items
    // - type items
    // ty_item_hir_decl,
    TypeMethodFnHirDecl,
    TypeMemoizedFieldHirDecl,
    TypeAssociatedFnHirDecl,
    TypeAssociatedValHirDecl,
    TypeAssociatedTypeHirDecl,
    // - trait items
    TraitAssociatedFnHirDecl,
    TraitAssociatedTypeHirDecl,
    TraitAssociatedValHirDecl,
    TraitMethodFnHirDecl,
    // - trait for type
    trai_for_ty_item_hir_decl,
    TraitForTypeAssociatedFnHirDecl,
    TraitForTypeAssociatedTypeHirDecl,
    TraitForTypeAssociatedValHirDecl,
    TraitForTypeMethodFnHirDecl,
    // ty
    ty_hir_decl,
    EnumTypeHirDecl,
    ExternTypeHirDecl,
    RecordTypeHirDecl,
    PropsStructTypeHirDecl,
    TupleStructTypeHirDecl,
    UnionHirDecl,
    UnitStructHirDecl,
    // trai
    trai_hir_decl,
    TraitHirDecl,
    // fugitive
    fugitive_hir_decl,
    FunctionFnFugitiveHirDecl,
    FunctionGnFugitiveHirDecl,
    TypeAliasHirDecl,
    ValFugitiveHirDecl,
    // ty variant
    ty_variant_hir_decl,
    EnumTupleVariantHirDecl,
    EnumPropsVariantHirDecl,
    EnumUnitTypeVariantHirDecl,
    // impl block
    // - type
    ty_impl_block_hir_decl,
    TypeImplBlockHirDecl,
    // - trait for type
    trai_for_ty_impl_block_hir_decl,
    TraitForTypeImplBlockHirDecl,
    // attr
    DeriveAttrHirDecl,
);
