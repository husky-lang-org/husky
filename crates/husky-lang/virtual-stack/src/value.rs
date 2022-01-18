use crate::any::Any;

use syntax_types::*;

use crate::*;

#[derive(Debug)]
pub enum VirtualStackValue<'stack> {
    Undefined,
    Primitive(PrimitiveValue),
    Owned(Box<dyn Any>),
    Ref(&'static dyn Any),
    MutRef(&'stack mut dyn Any),
}

impl<'stack> From<PrimitiveValue> for VirtualStackValue<'stack> {
    fn from(value: PrimitiveValue) -> Self {
        VirtualStackValue::Primitive(value)
    }
}

impl<'stack> From<&PrimitiveValue> for VirtualStackValue<'stack> {
    fn from(value: &PrimitiveValue) -> Self {
        VirtualStackValue::Primitive(*value)
    }
}

impl<'stack> Default for VirtualStackValue<'stack> {
    fn default() -> Self {
        todo!()
    }
}

impl<'stack> VirtualStackValue<'stack> {
    pub(super) fn as_input(&self, contract: InputContract) -> VirtualStackResult<Self> {
        match contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => todo!(),
        }
    }

    pub(super) fn as_primitive(&self) -> VirtualStackResult<PrimitiveValue> {
        if let Self::Primitive(value) = self {
            Ok(*value)
        } else {
            todo!()
        }
    }

    pub(super) fn as_i32(&self) -> VirtualStackResult<i32> {
        if let PrimitiveValue::I32(i) = self.as_primitive()? {
            Ok(i)
        } else {
            todo!()
        }
    }

    pub(super) fn as_f32(&self) -> VirtualStackResult<f32> {
        if let PrimitiveValue::F32(f) = self.as_primitive()? {
            Ok(f)
        } else {
            todo!()
        }
    }

    pub(super) fn as_u32(&self) -> VirtualStackResult<u32> {
        if let PrimitiveValue::B32(f) = self.as_primitive()? {
            Ok(f)
        } else {
            todo!()
        }
    }

    pub(super) fn as_bool(&self) -> VirtualStackResult<bool> {
        if let PrimitiveValue::Bool(b) = self.as_primitive()? {
            Ok(b)
        } else {
            todo!()
        }
    }

    pub fn clone_any(&self) -> Option<Box<dyn Any>> {
        todo!()
    }
}
