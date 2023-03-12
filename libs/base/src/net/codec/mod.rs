
mod concretes;
pub use concretes::*;

mod processor;
pub use processor::*;

//mod hexdump;
//pub use hexdump::*;

//use crate::error::ErrorCode;

use std::{sync::Arc, slice::Iter, iter::Rev};

use crate::{error::ErrorCode, buf::MemoryStream};

//--------------------------------------------------------------------------

pub trait Codec {
    fn encode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode>;
    fn decode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode>;
}

//--------------------------------------------------------------------------

pub(crate) trait CodecIterator {
    //fn next(&self, stream: &BufferedStream) -> Result<(), Error>;
    fn next(&mut self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode>;
}

//--------------------------------------------------------------------------

pub(crate) struct DecodeIterator<'a> {
    iter: Iter<'a, Box<dyn Codec>>,
}

impl<'a> DecodeIterator<'a> {
    #[inline]
    pub fn new(iter: Iter<'a, Box<dyn Codec>>) -> Self {
        Self { iter }
    }
}

impl<'a> CodecIterator for DecodeIterator<'a> {
    fn next(&mut self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        let codec = match self.iter.next() {
            Some(codec) => codec,
            None => return Ok(stream),
        };

        match codec.decode(stream) {
            Ok(stream) => self.next(stream),
            Err(e) => Err(e)
        }
    }
}

//--------------------------------------------------------------------------

pub(crate) struct EncodeIterator<'a> {
    iter: Rev<Iter<'a, Box<dyn Codec>>>,
}

impl<'a> EncodeIterator<'a> {
    #[inline]
    pub fn new(iter: Rev<Iter<'a, Box<dyn Codec>>>) -> Self {
        Self { iter }
    }
}

impl<'a> CodecIterator for EncodeIterator<'a> {
    fn next(&mut self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        let codec = match self.iter.next() {
            Some(codec) => codec,
            None => return Ok(stream),
        };

        match codec.encode(stream) {
            Ok(stream) => self.next(stream),
            Err(e) => Err(e)
        }
    }
}

//--------------------------------------------------------------------------