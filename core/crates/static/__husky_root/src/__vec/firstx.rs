use super::*;

pub static VEC_FIRST: EntityStaticDefn = EntityStaticDefn {
    name: "firstx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::Member(&MemberLinkage {
            copy_access: SpecificRoutineFp(generic_vec_first_copy),
            eval_ref_access: SpecificRoutineFp(generic_vec_first_eval_ref),
            temp_ref_access: SpecificRoutineFp(generic_vec_first_temp_ref),
            temp_mut_access: SpecificRoutineFp(generic_vec_first_mut),
            move_access: SpecificRoutineFp(generic_vec_first_move),
            nargs: 1,
            dev_src: static_dev_src!(),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_first_copy<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}

fn generic_vec_first_eval_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_ref();
    match generic_vec.first() {
        Some(value) => Ok(value.bind_eval_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_temp_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_ref();
    match generic_vec.first() {
        Some(value) => Ok(value.bind_temp_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_mut<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let (generic_vec, stack_idx, gen): (&mut VirtualVec<'eval>, _, _) =
        values[0].downcast_mut_full();
    match generic_vec.first_mut() {
        Some(value) => Ok(value.bind_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_move<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}
