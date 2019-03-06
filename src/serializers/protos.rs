
use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LabelMessage<'a> {
    pub label: Cow<'a, str>,
    pub msg: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for LabelMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.label = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.msg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LabelMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.label).len())
        + 1 + sizeof_len((&self.msg).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.label))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.msg))?;
        Ok(())
    }
}

