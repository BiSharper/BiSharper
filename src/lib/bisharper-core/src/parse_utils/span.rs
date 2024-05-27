use core::fmt;
use std::fmt::{Debug, Display};
use std::ops::Range;

pub type BSpanned<T> = (BSpan, T);
pub struct BSpan {
    pub start: usize,
    pub stop: usize
}

impl From<Range<usize>> for BSpan {
    fn from(range: Range<usize>) -> Self {
        BSpan {
            start: range.start,
            stop: range.end,
        }
    }
}

impl Debug for BSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.start, self.stop)
    }
}

impl Display for BSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.start, self.stop)
    }
}
