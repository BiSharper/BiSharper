use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use chumsky::prelude::Input;
use crate::parse_utils::{BParseResult};

#[cfg(feature = "preprocessor-rv")]
pub mod rv;
#[cfg(feature = "preprocessor-enf")]
pub mod enfusion;



pub trait PreProcessor<'ast>: Sized {
    type Context: 'ast;

    type Input: 'ast;

    type Output: 'ast;

    type Error: Clone + PartialEq + Debug;

    fn process(input: Self::Input, ctx: &mut Self::Context) -> BParseResult<Self::Output, Self::Error>;
}


