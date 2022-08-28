use super::*;
use cyclic_slice::CyclicSlice;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualCyclicSlice<'eval> {
    pub data: CyclicSlice<'eval, __Register<'eval>>,
}

impl<'eval> std::ops::Deref for VirtualCyclicSlice<'eval> {
    type Target = CyclicSlice<'eval, __Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualCyclicSlice<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for VirtualCyclicSlice<'eval> {
    type __StaticSelf = VirtualCyclicSlice<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "CyclicSlice<Any>".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for VirtualCyclicSlice<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
