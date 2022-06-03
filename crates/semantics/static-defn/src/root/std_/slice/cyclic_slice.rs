use vm::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CyclicSlice<'a, T> {
    pub start: i32,
    pub end: i32,
    pub total: &'a [T],
}

impl<'a, T> CyclicSlice<'a, T> {
    pub fn first(&self) -> Option<&T> {
        todo!()
    }
    pub fn last(&self) -> Option<&T> {
        todo!()
    }
    pub fn first_mut(&mut self) -> Option<&mut T> {
        todo!()
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        todo!()
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValue<'eval> for CyclicSlice<'eval, T> {
    fn static_type_id() -> StaticTypeId {
        todo!()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }
}

pub static STD_SLICE_CYCLIC_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "CyclicSlice",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "std::slice::CyclicSlice",
        generic_parameters: &[StaticGenericPlaceholder {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        ty_members: &[
            &STD_SLICE_CYCLIC_SLICE_START_DEFN,
            &STD_SLICE_CYCLIC_SLICE_END_DEFN,
            &STD_SLICE_CYCLIC_SLICE_FIRST_DEFN,
            &STD_SLICE_CYCLIC_SLICE_LAST_DEFN,
        ],
        static_trait_impls: &[],
        variants: &[],
        kind: TyKind::Struct,
        visualizer: StaticVisualizer::Vec,
        opt_type_call: None,
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_START_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "start",
    subscopes: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        ty: "i32",
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_END_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "end",
    subscopes: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        ty: "i32",
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_FIRST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: linkage!(generic_cyclic_slice_first_copy, 1),
                ref_access: linkage!(generic_cyclic_slice_first_ref, 1),
                ref_mut_access: linkage!(generic_cyclic_slice_first_mut, 1),
                move_access: linkage!(generic_cyclic_slice_first_move, 1),
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_cyclic_slice_first_copy<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    todo!()
}

fn generic_cyclic_slice_first_ref<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let generic_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    match generic_cyclic_slice.first() {
        Some(value) => Ok(value.stack_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_mut<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let (generic_cyclic_slice, stack_idx, gen): (
        &mut CyclicSlice<'eval, MemberValue<'eval>>,
        _,
        _,
    ) = values[0].downcast_mut_full();
    match generic_cyclic_slice.first_mut() {
        Some(value) => Ok(value.stack_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_move<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    todo!()
}

pub static STD_SLICE_CYCLIC_SLICE_LAST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "last",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: linkage!(generic_cyclic_slice_last_copy, 1),
                ref_access: linkage!(generic_cyclic_slice_last_ref, 1),
                ref_mut_access: linkage!(generic_cyclic_slice_last_mut, 1),
                move_access: linkage!(generic_cyclic_slice_last_move, 1),
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_cyclic_slice_last_copy<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    todo!()
}

fn generic_cyclic_slice_last_ref<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let generic_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    match generic_cyclic_slice.last() {
        Some(value) => Ok(value.stack_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_mut<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let (generic_cyclic_slice, stack_idx, gen): (
        &mut CyclicSlice<'eval, MemberValue<'eval>>,
        _,
        _,
    ) = values[0].downcast_mut_full();
    match generic_cyclic_slice.last_mut() {
        Some(value) => Ok(value.stack_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_move<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    todo!()
}
