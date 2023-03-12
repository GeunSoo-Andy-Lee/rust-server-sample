use std::sync::Arc;

use base::{net::Codec, buf::MemoryStream, error::ErrorCode};


pub struct CodecA { }
impl Codec for CodecA {
    fn encode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        println!("CodecA::encode");

        //make_error!(ErrorCode::MalformedStream)
        //codec.next(stream)
        Ok(stream)
    }
    //fn decode(&self, stream: &Arc<MemoryStream>, decode: &DecodePipeline) -> Result<(), Error> {
    fn decode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        println!("CodecA::decode");

        Ok(stream)
    }
}

pub struct CodecB { }
impl Codec for CodecB {
    fn encode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        println!("CodecB::encode");

        //make_error!(ErrorCode::MalformedStream)
        //codec.next(stream)
        Ok(stream)
    }
    //fn decode(&self, stream: &BufferedStreame: &DecodePipeline) -> Result<(), Error> {
    fn decode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        println!("CodecB:decode");

        //Err()
        //make_error!(ErrorCode::MalformedStream)
        //codec.next(stream);

        //Err(ErrorCode::NotEnoughStream)

        //let mut buf = stream.clone();
        //buf.push(b'B');

        Ok(stream)
    }
}

pub struct CodecC { }
impl Codec for CodecC {
    fn encode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        println!("CodecC::encode");

        //make_error!(ErrorCode::MalformedStream)
        //codec.next(stream)
        Ok(stream)
    }
    //fn decode(&self, stream: &Arc<MemoryStream>, decode: &DecodePipeline) -> Result<(), Error> {
    fn decode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        println!("CodecC:decode");

        //make_error!(ErrorCode::MalformedStream)
        //codec.next(stream);
        //Ok(Arc<MemoryStream>::new())
        Ok(stream)
    }
}