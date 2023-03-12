use std::sync::Arc;

use crate::{net::Codec, error::{Error, ErrorCode}, buf::MemoryStream};


pub struct HexDump { }
impl Codec for HexDump {
    fn encode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        //let mut buffer = Vec::new();

        //hxdmp::hexdump(next.as_slice(), &mut buffer);
        //log::trace!("{}", String::from_utf8_lossy(&buffer));
        //next.process(stream)

        Ok(stream)
    }
    //fn decode(&self, stream: &BufferedStream, decode: &DecodePipeline) -> Result<(), Error> {
    fn decode(&self, stream: Arc<MemoryStream>) -> Result<Arc<MemoryStream>, ErrorCode> {
        //let mut buffer = Vec::new();

        //hxdmp::hexdump(next.as_slice(), &mut buffer);
        //log::trace!("{}", String::from_utf8_lossy(&buffer));
        //next.process(stream)

        Ok(stream)
    }
}
