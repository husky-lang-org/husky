use std::borrow::Cow;

use super::*;

impl<'temp, T> HasStaticTypeInfo for &'temp [T]
where
    T: HasStaticTypeInfo,
{
    type StaticSelf = &'static [T::StaticSelf];
    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'temp, 'eval, 'a: 'eval, T: __AnyValue<'a> + 'temp> __AnyValue<'eval> for &'temp [T] {
    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
        // serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        todo!()
    }
    fn print_short(&self) -> String {
        todo!()
    }
}

fn gen_iter<'temp, 'eval: 'temp, T>(
    slice: &'temp [T],
) -> Box<dyn Iterator<Item = __TempValue<'temp, 'eval>> + 'temp>
where
    T: __AnyValueDyn<'eval> + 'eval,
{
    Box::new(slice.iter().map(|t| __TempValue::TempRefEval(t)))
}
