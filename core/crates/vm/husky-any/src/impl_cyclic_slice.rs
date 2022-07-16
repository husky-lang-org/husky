use cyclic_slice::CyclicSlice;
use husky_entity_route::{entity_route_menu, make_route};
use thin_vec::thin_vec;
use word::RootIdentifier;

use super::*;

impl<'a, T> __HasStaticTypeInfo for CyclicSlice<'a, T>
where
    T: __HasStaticTypeInfo,
{
    type __StaticSelf = CyclicSlice<'static, T::__StaticSelf>;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        format!("CyclicSlice<{}>", T::__static_type_name()).into()
    }
}

impl<'eval, 'a: 'eval, 'b: 'eval, T: __AnyValue<'a>> __AnyValue<'eval> for CyclicSlice<'b, T> {
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(self.iter().map(|elem| elem.__to_json_value()).collect())
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        husky_entity_route::ty_route_with::<Self::__StaticSelf>(|| {
            make_route(
                entity_route_menu().std_slice_cyclic_slice,
                thin_vec![T::__static_ty().into()],
            )
        })
    }

    fn __print_short(&self) -> String {
        format!(
            "{{ start: {}, end: {}, data: {} }}",
            self.start,
            self.end,
            print_sequence(
                "{ ",
                self.iter(),
                &|value| format!("{}", value.__print_short()),
                " }",
                20,
            )
        )
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Group(
            self.iter()
                .enumerate()
                .map(|(i, element)| visualize_element(i, element.__short()))
                .collect::<__EvalResult<Vec<_>>>()?,
        )))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        todo!()
    }
}
