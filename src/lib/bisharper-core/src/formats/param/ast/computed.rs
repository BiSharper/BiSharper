use std::collections::HashMap;
use crate::param::{ParamArray, ParamClassBase, ParamLiteralBase};
use crate::param::ast::patch::ParamStringSyntax;

pub struct ParamClass<'mem> {
    base: Option<&'mem ParamClass<'mem>>,
    params: HashMap<String, ParamComputedLiteral>,
    classes: HashMap<String, ParamClass<'mem>>
}

impl<'mem> ParamClassBase<ParamComputedLiteral> for ParamClass<'mem> {
    fn name(&self) -> &str {
        todo!()
    }

    fn base_name(&self) -> Option<&'mem str> {
        Some(self.base?.name())
    }

    fn property(&self, name: &str) -> Option<&ParamComputedLiteral> { self.params.get(name) }
}

pub enum ParamComputedLiteral {
    I32(i32), I64(i64), F32(f32),
    String(String),
    Array(ParamArray<Self>),
}

impl ParamLiteralBase for ParamComputedLiteral {
    fn f32(f32: f32) -> Self { Self::F32(f32) }

    fn i32(i32: i32) -> Self { Self::I32(i32) }

    fn i64(i64: i64) -> Self { Self::I64(i64) }

    fn array(array: ParamArray<Self>) -> Self { Self::Array(array) }

    fn string(string: String) -> Self { Self::String(string) }
}



#[cfg(feature = "param-patch")]
impl From<ParamStringSyntax> for String  {
    fn from(value: ParamStringSyntax) -> Self { value.inner }
}



