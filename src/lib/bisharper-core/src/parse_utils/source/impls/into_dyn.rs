use std::marker::PhantomData;
use crate::parse_utils::{BDynamicSource, BIterator, BIteratorBase, BSeekableSource, BStaticSource};

pub struct BIntoDynamicSource<T, S: BStaticSource<T>> {
    inner: S,
    position: usize,
    _phantom: PhantomData<T>
}

impl<T, S: BStaticSource<T>> BIntoDynamicSource<T, S> {
    pub fn create(inner: S, position: usize) -> Self {
        BIntoDynamicSource { inner, position, _phantom: PhantomData, }
    }

    pub fn into_inner(self) -> S { self.inner }

    pub fn unwrap(self) -> (usize, S) { (self.position, self.inner) }

    pub fn as_inner(&self) -> &S { &self.inner }

    pub fn as_inner_mut(&mut self) -> &mut S { &mut self.inner }
}

impl<T, S: BStaticSource<T>> BIteratorBase for BIntoDynamicSource<T, S> {
    type Error = S::Error;
}

impl<T, S: BStaticSource<T>> BIterator<T> for BIntoDynamicSource<T, S> {
    type Token = S::Token;
}

impl<T, S: BStaticSource<T>> From<S> for BIntoDynamicSource<T, S> {
    fn from(inner: S) -> Self { Self::create(inner, 0) }
}

impl<T, S: BStaticSource<T>> BSeekableSource<T, S::Error> for BIntoDynamicSource<T, S> {
    fn b_seek(&mut self, position: usize) -> Result<(), S::Error> {
        self.position = position;
        Ok(())
    }
}

impl<T, S: BStaticSource<T>> BDynamicSource<T> for BIntoDynamicSource<T, S> {
    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error> {
        let (next_pos, next_token) = self.inner.b_next_at(self.position)?;
        self.position = next_pos;
        return Ok(next_token);
    }

    fn b_position(&self) -> usize { self.position }
}