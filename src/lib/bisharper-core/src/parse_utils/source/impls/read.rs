use std::io;
use std::io::Read;
use std::marker::PhantomData;

use crate::parse_utils::{BDynamicSource, BIterator, BIteratorBase, BSeekableSource};

pub trait BReadableToken<R: Read, E: From<io::Error>>: Sized {
    fn b_read(reader: &mut R) -> Result<Option<Self>, E>;
}

impl<R: Read, E: From<io::Error>> BReadableToken<R, E> for u8 {
    fn b_read(reader: &mut R) -> Result<Option<Self>, E> {
        match reader.read_exact(&mut [0u8; 1]) {
            Ok(()) => Ok(Some(0)),
            Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            Err(err) => Err(E::from(err)),
        }
    }
}

impl<R: Read, E: From<io::Error>> BReadableToken<R, E> for char {
    fn b_read(reader: &mut R) -> Result<Option<Self>, E> {
        let mut buf = [0; 4];
        for i in 0..4 {
            match reader.read(&mut buf[i..i+1])? {
                0 if i == 0 => return Ok(None),
                0 => return Err(E::from(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete UTF-8 char"))),
                _ => if let Ok(s) = std::str::from_utf8(&buf[..=i]) {
                    if let Some(c) = s.chars().next() {
                        return Ok(Some(c));
                    }
                }
            }
        }
        Err(E::from(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence")))
    }
}

pub type BSimpleSourceStream<T, R> = BSourceStream<T, R, io::Error>;

pub type BCharStream<R> = BSimpleSourceStream<char, R>;
pub type BByteStream<R> = BSimpleSourceStream<u8, R>;

pub struct BSourceStream<T: BReadableToken<R, E>, R: Read, E: From<io::Error>> {
    inner: R,
    position: usize,
    _token: PhantomData<T>,
    _error: PhantomData<E>
}

impl<T: BReadableToken<R, E>, R: Read, E: From<io::Error>> BDynamicSource<T> for BSourceStream<T, R, E> {
    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error> { T::b_read(&mut self.inner) }

    fn b_position(&self) -> usize { self.position }
}
impl<T: BReadableToken<R, E>, R: Read, E: From<io::Error>> BIteratorBase for BSourceStream<T, R, E> {
    type Error = E;
}

impl<T: BReadableToken<R, E>, R: Read, E: From<io::Error>> BIterator<T> for BSourceStream<T, R, E> {
    type Token = T;
}

impl<T: BReadableToken<R, E>, R: Read, E: From<io::Error>> BSeekableSource<T, E> for BSourceStream<T, R, E>{
    fn b_seek(&mut self, position: usize) -> Result<(), E> {
        self.position = position;
        Ok(())
    }
}
