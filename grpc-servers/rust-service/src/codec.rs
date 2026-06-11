use bytes::{Buf, BufMut, Bytes};
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::Status;

pub struct FlatMessage(pub Bytes);

#[derive(Clone, Default)]
pub struct FlatCodec;

impl Codec for FlatCodec {
    type Encode = FlatMessage;
    type Decode = FlatMessage;
    type Encoder = FlatEncoder;
    type Decoder = FlatDecoder;
    fn encoder(&mut self) -> FlatEncoder { FlatEncoder }
    fn decoder(&mut self) -> FlatDecoder { FlatDecoder }
}

pub struct FlatEncoder;
impl Encoder for FlatEncoder {
    type Item  = FlatMessage;
    type Error = Status;
    fn encode(&mut self, item: FlatMessage, buf: &mut EncodeBuf<'_>) -> Result<(), Status> {
        buf.put(item.0);
        Ok(())
    }
}

pub struct FlatDecoder;
impl Decoder for FlatDecoder {
    type Item  = FlatMessage;
    type Error = Status;
    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<FlatMessage>, Status> {
        let len = buf.remaining();
        if len == 0 { return Ok(None); }
        Ok(Some(FlatMessage(buf.copy_to_bytes(len))))
    }
}