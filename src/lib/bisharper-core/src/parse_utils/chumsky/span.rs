use std::ops::Range;
use chumsky::extra::ParserExtra;
use chumsky::prelude::{Input};
use chumsky::span::Span;
use crate::parse_utils::span::BSpan;


impl Span for BSpan {
    type Context = ();
    type Offset = usize;

    fn new(_context: Self::Context, range: Range<Self::Offset>) -> Self { range.into() }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset { self.start }

    fn end(&self) -> Self::Offset { self.stop }
}