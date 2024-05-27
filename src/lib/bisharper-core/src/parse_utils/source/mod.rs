//! This module defines traits and implementations for working with various types of data sources in
//! an indexable or iterator-like fashion.
//!
//! # Purpose and Goals
//! The primary goal of this module is to provide a generic interface for interacting with different
//! types of data sources, such as memory slices, strings, and stream types like Read. It abstracts
//! away the specifics of each source type, allowing users to interact with them uniformly. Some of
//! the key objectives include:
//!
//! - **Abstraction for Data Sources:** Provide a unified interface for working with different data
//! sources in a unified manner.
//! - **Streaming and Iteration:** Facilitate streaming and iteration over data sources even if they
//! do not inherently support it.
//! - **Error Handling:** Gracefully handle errors specific to each data source without enforcing
//! too many annoying hardcoded errors, providing clear error types for users.
//!
//!
//! # Source Types
//! This module defines several traits representing different types of data sources and their
//! characteristics, some of which have coined their own name for easier referencing
//! (e.g. the static/dynamic duo). Some of the key traits are as follows:
//! - **Static Source (`BStaticSource<T>`):** Represents data that can be indexed with a position
//! using a fixed starting offset. These sources usually represent something like a slice or string
//! meaning they are also Reference Sources most of the time.
//! - **Dynamic Source (`BDynamicSource<T>`):** Represents data with a position, acts like a normal
//! iterator type where a position is kept and a `next` method is provided which increments the
//! position.
//!

mod impls; pub use impls::*;
mod lexer; pub use lexer::*;
use std::borrow::Borrow;
use std::io;
use std::io::{Read};
use std::ops::Range;
use thiserror::Error;

pub trait BIteratorBase {
    type Error;
}
pub trait BIterator<T>: BIteratorBase {
    type Token: Borrow<T>;
}

pub trait BKnownEndSource: BIteratorBase {
    fn b_end_offset(&self) -> usize;

    fn b_readable_length(&self) -> usize { self.b_end_offset() }

    fn b_length(&self) -> usize { self.b_end_offset() }
}

pub trait BStaticSource<T>: BIterator<T> {
    fn b_start_offset(&self) -> usize;

    fn b_dynamic(self) -> BIntoDynamicSource<T, Self> where Self: Sized { BIntoDynamicSource::from(self) }

    fn b_actual_start(&self) -> usize { self.b_start_offset() }

    fn b_next_at(&mut self, offset: usize) -> Result<(usize, Option<Self::Token>), Self::Error>;
}

pub trait BDynamicSource<T>: BIterator<T> {
    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error>;

    fn b_peekable(self) -> BIntoPeekableSource<T, Self> where Self: Sized {
        BIntoPeekableSource::from(self)
    }

    fn b_position(&self) -> usize;
}

pub trait BPeekableSource<T>: BDynamicSource<T> {
    fn b_peek(&mut self, position: usize) -> Result<(usize, Option<Self::Token>), Self::Error>;
}

pub trait BSeekableSource<T, E>: BDynamicSource<T> {
    fn b_seek(&mut self, position: usize) -> Result<(), E>;
}

pub trait BReferenceSource<'a, T: 'a>: BIterator<T, Token = &'a T> {}

pub trait BValueSource<T>: BIterator<T, Token = T> {}

#[derive(Debug, Error)]
pub enum BSliceError {
    #[error("Invalid range supplied for source!")]
    InvalidRange
}

pub trait BStrSource<E: From<io::Error>>: BStaticSource<char> + BStaticSource<u8> {

}

pub trait BOwnedStrSource<'a, E: From<io::Error> + From<BSliceError>>: BStrSource<E> + BSliceSource<&'a str, E> {

}

pub trait BSliceSource<T, E: From<BSliceError>>: BIteratorBase<Error = E> + BKnownEndSource {
    fn b_full_slice(&self) -> Result<T, Self::Error>;

    fn b_slice(&self, range: Range<usize>) -> Result<T, Self::Error>;
}

