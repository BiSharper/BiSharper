use std::io;
use std::ops::Range;
use thiserror::Error;
use crate::parse_utils::{BIterator, BIteratorBase, BKnownEndSource, BOwnedStrSource, BSliceError, BSliceSource, BStaticSource, BStrSource, BValueSource};

#[derive(Debug, Error)]
pub enum BStrError {
    #[error("IO Error")]
    IO(#[from] io::Error),
    #[error("Slice Error")]
    Slice(#[from] BSliceError)
}

impl<'a> BOwnedStrSource<'a, BStrError> for &'a str {

}

impl<'a> BStrSource<BStrError> for &'a str {

}

impl<'a> BIteratorBase for &'a str {
    type Error = BStrError;
}

impl<'a, T> BIterator<T> for &'a str {
    type Token = T;
}

impl<'a, T> BValueSource<T> for &'a str {}

impl<'a> BSliceSource<&'a str, BStrError> for &'a str {
    fn b_full_slice(&self) -> Result<&'a str, Self::Error> { Ok(&self) }

    fn b_slice(&self, range: Range<usize>) -> Result<&'a str, Self::Error> {
        self.get(range).ok_or(BSliceError::InvalidRange.into())
    }
}

impl<'a> BKnownEndSource for &'a str {
    fn b_end_offset(&self) -> usize { self.len() }
}

impl<'a> BStaticSource<char> for &'a str {
    fn b_start_offset(&self) -> usize { 0 }

    fn b_next_at(&mut self, offset: usize) -> Result<(usize, Option<Self::Token>), Self::Error> {
        if offset >= self.len() {
            return Ok((offset, None));
        }

        let mut indices = self[offset..].char_indices();

        if let Some((x, char)) = indices.next() {
            return Ok((offset + x + char.len_utf8(), Some(char)))
        }

        Ok((offset, None))
    }
}

impl<'a> BStaticSource<u8> for &'a str {
    fn b_start_offset(&self) -> usize { 0 }

    fn b_next_at(&mut self, offset: usize) -> Result<(usize, Option<Self::Token>), Self::Error> {
        if offset >= self.len() {
            return Ok((offset, None));
        }

        let next = self.as_bytes()[offset];

        return Ok((offset + 1, Some(next)))
    }
}