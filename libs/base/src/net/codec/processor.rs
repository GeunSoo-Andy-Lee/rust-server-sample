use std::sync::Arc;

use crate::error::{Error, ErrorCode};
use crate::buf::MemoryStream;

use super::{Codec, CodecIterator, DecodeIterator, EncodeIterator};


pub struct Processor {
    codec: Vec<Box::<dyn Codec + 'static>>
}

unsafe impl Send for Processor {}
unsafe impl Sync for Processor {}

impl Processor {
    pub fn new() -> Self {
        Self {
            codec: Vec::new()
        }
    }

    pub fn add<T: Codec + 'static>(&mut self, codec: T) -> &mut Self {
        self.codec.push(Box::new(codec));
        self
    }

    pub(crate) fn inbound(&self, buffer: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        //Err(ErrorCode::NotEnoughStream)
        DecodeIterator::new(self.codec.iter()).next(buffer)
    }

    pub(crate) fn outbound(&self, buffer: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        //Err(ErrorCode::NotEnoughStream)
        EncodeIterator::new(self.codec.iter().rev()).next(buffer)
    }
}

//////////////////////////////////////////////////////////////////////
/* 
struct Pipeline {
    codec: Vec<Box::<dyn Codec>>
}

impl Pipeline {
    pub fn add<T: Codec + 'static>(&mut self, codec: T) -> &mut Self {
        self.codec.push(Box::new(codec));
        self
    }

    fn inbound(&self, buffer: &Vec<u8>) {
        let res = DecodePipeline::new(&self.codec).next(buffer);
        println!("{:?}", res);
    }

    fn outbound(&self, buffer: &Vec<u8>) {
        let res = EncodePipeline::new(&self.codec).next(buffer);
        println!("{:?}", res);
    }
}
*/