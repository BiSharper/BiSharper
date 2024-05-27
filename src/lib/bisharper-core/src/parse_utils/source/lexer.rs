// use std::marker::PhantomData;
// use std::mem;
// use crate::parse_utils::{BKnownStartSource, BSource, BSourceBase};
//
//
// pub trait BSourceIterator<T, S: BSourceBase<T>>  {
//     fn b_source(&self) -> &S;
//
//     fn b_source_mut(&mut self) -> &mut S;
//
//     fn b_position(&self) -> usize;
//
//     fn b_seek(&mut self, offset: usize) -> Result<(), S::Error>;
//
//     fn b_next(&mut self) -> Result<Option<S::TokenMaybe>, S::Error>;
//
//     fn b_peekable(self) -> BPeekable<T, S, Self> where Self: Sized {
//         BPeekable {
//             inner: self,
//             next: None,
//         }
//     }
// }
//
// pub struct BLexer<'src, T, S: BSourceBase<T>> {
//     inner: S,
//     position: usize,
//     _phantom: PhantomData<&'src T>
// }
//
// impl<'src, T, S: BSource<'src, T> + BKnownStartSource> From<S> for BLexer<'src, T, S> {
//     fn from(inner: S) -> Self {
//         let position = inner.b_start_offset();
//         Self::create(inner, position)
//     }
// }
//
// impl<'src, T, S: BSource<'src, T>> BLexer<'src, T, S> {
//     pub fn create(inner: S, position: usize) -> Self {
//         BLexer { inner, position, _phantom: PhantomData }
//     }
// }
// impl<'src, T, S: BSource<'src, T>> BSourceIterator<T, S> for BLexer<'src, T, S> {
//     fn b_source(&self) -> &S { &self.inner }
//
//     fn b_source_mut(&mut self) -> &mut S { &mut self.inner }
//
//     fn b_position(&self) -> usize { self.position }
//
//     fn b_seek(&mut self, offset: usize) -> Result<(), S::Error> { self.position = offset; Ok(()) }
//
//     fn b_next(&mut self) -> Result<Option<S::TokenMaybe>, S::Error> {
//         let (next_offset, next_token) = self.inner.b_next_at(self.position)?;
//         self.b_seek(next_offset)?;
//         Ok(next_token)
//     }
// }
//
// pub struct BPeekable<T, S: BSourceBase<T>, CS: BSourceIterator<T, S> > {
//     inner: CS,
//     next: Option<(usize, Option<S::TokenMaybe>)>,
// }
//
// impl<'src, T, S: BSource<'src, T>, CS: BSourceIterator<T, S>> BSourceIterator<T, S> for BPeekable<T, S, CS> {
//     fn b_source(&self) -> &S { self.inner.b_source() }
//
//     fn b_source_mut(&mut self) -> &mut S { self.inner.b_source_mut() }
//
//     fn b_position(&self) -> usize { self.inner.b_position() }
//
//     fn b_seek(&mut self, offset: usize) -> Result<(), S::Error> {
//         self.next = None;
//         self.inner.b_seek(offset)
//     }
//
//     fn b_next(&mut self) -> Result<Option<S::TokenMaybe>, S::Error> {
//         if self.next.is_some() {
//             let mut peeked = None;
//             mem::swap(&mut self.next, &mut peeked);
//
//             let (next_offset, next_token) = peeked.unwrap();
//             self.inner.b_seek(next_offset)?;
//
//             return Ok(next_token)
//         }
//
//         self.inner.b_next()
//     }
// }
//
// impl<'src, T, S: BSource<'src, T>, CS: BSourceIterator<T, S>> BPeekable<T, S, CS> {
//     fn b_peek(&mut self) -> Result<&(usize, Option<S::TokenMaybe>), S::Error> {
//         if self.next.is_some()  {
//             return Ok(self.next.as_ref().unwrap());
//         }
//
//         let position = self.b_position();
//         self.next = Some(self.b_source_mut().b_next_at(position)?);
//         return Ok(self.next.as_ref().unwrap());
//     }
// }