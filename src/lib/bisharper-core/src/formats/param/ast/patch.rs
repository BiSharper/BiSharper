use crate::param::{ParamArray, ParamClassBase, ParamLiteralBase};

pub struct ParamClassSyntax {
    name: String,
    parent: Option<String>,
    statements: Vec<ParamStatementSyntax>
}

impl ParamClassBase<ParamPatchLiteral> for ParamClassSyntax {
    fn name(&self) -> &str { &self.name }

    fn base_name(&self) -> Option<&str> {
        self.parent.as_ref().map(|s| s.as_str())
    }

    fn property(&self, name: &str) -> Option<&ParamPatchLiteral> {
        todo!()
    }
}

impl ParamLiteralBase for ParamPatchLiteral {
    fn db(f64: f64) -> Self { Self::F64(f64) }

    fn f32(f32: f32) -> Self { Self::F32(f32) }

    fn i32(i32: i32) -> Self { Self::I32(i32) }

    fn i64(i64: i64) -> Self { Self::I64(i64) }

    fn array(array: ParamArray<Self>) -> Self { Self::Array(array) }

    fn string(string: String) -> Self { Self::String(ParamStringSyntax { inner: string, quoted: true }) }
}

pub struct ParamParameterSyntax {
    name: String,
    operator: ParamOperatorSyntax,
    value: ParamPatchLiteral
}

pub struct ParamStringSyntax {
    pub inner: String,
    pub quoted: bool
}

pub enum ParamPatchLiteral {
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    String(ParamStringSyntax),
    Array(ParamArray<Self>),
    Reference(String),
    Evaluation(String)
}


impl ParamPatchLiteral {
    fn reference(reference: String) -> Self { Self::Reference(reference) }

    fn evaluation(expression: String) -> Self { Self::Evaluation(expression) }
}

pub enum ParamStatementSyntax {
    Enum(),
    Parameter(ParamParameterSyntax),
    Execute(String),
    Patch(ParamClassSyntax)
}

pub enum ParamOperatorSyntax {
    AddAssign, SubAssign, Assign
}