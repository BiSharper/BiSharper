use crate::param::calculate_db;

#[cfg(feature = "param-computed")]
mod computed; pub use computed::*;
#[cfg(feature = "param-patch")]
mod patch; pub use patch::*;

pub trait ParamClassBase<L: ParamLiteralBase> {
    fn name(&self) -> &str;

    fn base_name(&self) -> Option<&str>;

    fn property(&self, name: &str) -> Option<&L>;
}

pub trait ParamLiteralBase : Sized {
    fn db(f64: f64) -> Self { Self::f32(calculate_db(f64)) }

    fn f32(f32: f32) -> Self;

    fn i32(i32: i32) -> Self;

    fn i64(i64: i64) -> Self;

    fn array(array: ParamArray<Self>) -> Self;

    fn string(string: String) -> Self;
}


pub struct ParamArray<L: ParamLiteralBase> {
    inner: Vec<L>
}