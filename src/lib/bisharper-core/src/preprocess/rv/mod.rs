// use std::marker::PhantomData;
// use crate::parse_utils::{BParseResult, BSpanned, BSource};
// use crate::preprocess::PreProcessor;
// use crate::preprocess::rv::driver::RvContext;
// use crate::preprocess::rv::error::RvProcessingError;
// use crate::preprocess::rv::lexer::RvToken;
//
// mod lexer;
// mod driver;
// mod error;
//
// pub struct RvOutput;
//
// pub struct RvProcessor<'ast, S: BSource<'ast, char>> {
//     stream: S,
//     phantom_data: PhantomData<&'ast S>
// }
//
// impl<'ast, S: BSource<'ast, char>> PreProcessor<'ast> for RvProcessor<'ast, S> {
//     type Context = RvContext;
//     type Input = S;
//     type Output = RvOutput;
//     type Error = RvProcessingError;
//
//     fn process(input: Self::Input, ctx: &mut Self::Context) -> BParseResult<Self::Output, Self::Error> {
//         let processor = RvProcessor {
//             stream: input,
//             phantom_data: PhantomData,
//         };
//
//         todo!()
//     }
// }