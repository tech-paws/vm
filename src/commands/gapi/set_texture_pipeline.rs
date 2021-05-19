use crate::commands::gapi;
use crate::commands::{CommandNew, Source};
use vm_buffers::{BytesReader, BytesWriter};

pub struct Command {
    pub id: u64,
}

impl Command {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl CommandNew<u64> for Command {
    fn id() -> u64 {
        gapi::SET_TEXTURE_PIPELINE
    }

    fn source() -> Source {
        Source::GAPI
    }

    fn write(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_u64(self.id);
    }

    fn read(bytes_reader: &mut BytesReader) -> u64 {
        bytes_reader.read_u64()
    }
}
