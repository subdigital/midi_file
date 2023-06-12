use crate::byte_iter::ByteIter;
use crate::error::LibResult;
use crate::scribe::Scribe;
use std::io::{Read, Write};

// TODO - implement sysex messages
/// Caution: Sysex messages are [not implemented](https://github.com/webern/midi_file/issues/7) and
/// will error.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SysexEvent {
    t: SysexEventType,
    data: Vec<u8>,
}

impl SysexEvent {
    // TODO - implement a `new` function.
    // TODO - implement getter functions.

    pub(crate) fn parse<R: Read>(_first_byte: u8, _r: &mut ByteIter<R>) -> LibResult<Self> {
        noimpl!("SysexEvent::parse")
    }

    pub(crate) fn write<W: Write>(&self, _w: &mut Scribe<W>) -> LibResult<()> {
        noimpl!("SysexEvent::write")
    }
}

#[repr(u8)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Default)]
pub enum SysexEventType {
    #[default]
    F0 = 0xf0,
    F7 = 0xf7,
}
