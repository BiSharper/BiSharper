use crate::parse_utils::{BDynamicSource};

pub struct BIntoPeekableSource<T, S: BDynamicSource<T>> {
    inner: S,
    next: Option<(usize, Option<T>)>,
}

impl<T, S: BDynamicSource<T>> BIntoPeekableSource<T, S> {
    pub fn create(inner: S, next: Option<(usize, Option<T>)>) -> Self {
        BIntoPeekableSource { inner, next }
    }

    pub fn into_inner(self) -> S { self.inner }

    pub fn unwrap(self) -> (S, Option<(usize, Option<T>)>) { (self.inner, self.next) }

    pub fn as_inner(&self) -> &S { &self.inner }

    pub fn as_inner_mut(&mut self) -> &mut S { &mut self.inner }
}

impl<T, S: BDynamicSource<T>> From<S> for BIntoPeekableSource<T, S> {
    fn from(inner: S) -> Self { Self::create(inner, None) }
}
