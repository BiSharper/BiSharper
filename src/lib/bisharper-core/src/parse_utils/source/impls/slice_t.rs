use std::ops::Range;
use crate::parse_utils::{BIterator, BIteratorBase, BKnownEndSource, BReferenceSource, BSliceError, BSliceSource, BStaticSource};

impl<'a, T> BIteratorBase for &'a [T] {
    type Error = BSliceError;
}

impl<'a, T> BIterator<T> for &'a [T] {
    type Token = &'a T;
}

impl<'a, T> BReferenceSource<'a, T> for &'a [T] {}

impl<'a, T> BKnownEndSource for &'a [T] {
    fn b_end_offset(&self) -> usize { self.len() }
}

impl<'a, T> BSliceSource<&'a [T], BSliceError> for &'a [T] {

    fn b_full_slice(&self) -> Result<&'a [T], Self::Error> { Ok(&self) }

    fn b_slice(&self, range: Range<usize>) -> Result<&'a [T], Self::Error> {
        self.get(range).ok_or(BSliceError::InvalidRange)
    }
}

impl<'a, T> BStaticSource<T> for &'a [T] {
    fn b_start_offset(&self) -> usize { 0 }

    fn b_next_at(&mut self, offset: usize) -> Result<(usize, Option<Self::Token>), Self::Error> {
        if self.len() >= offset {
            return Ok((offset, None))
        }
        return Ok((offset + 1, Some(&self[offset])))
    }
}