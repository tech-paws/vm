use crate::commands::gapi;
use crate::commands::{CommandNew, Source};
use vm_buffers::{BytesReader, BytesWriter, IntoVMBuffers};
use vm_math::Vec4f;

pub struct Command {
    pub color: Vec4f,
}

impl Command {
    pub fn new(color: Vec4f) -> Self {
        Self { color }
    }
}

impl CommandNew<Vec4f> for Command {
    fn id() -> u64 {
        gapi::SET_COLOR_PIPELINE
    }

    fn source() -> Source {
        Source::GAPI
    }

    fn write(&self, bytes_writer: &mut BytesWriter) {
        self.color.write_to_buffers(bytes_writer);
    }

    fn read(bytes_reader: &mut BytesReader) -> Vec4f {
        Vec4f::read_from_buffers(bytes_reader)
    }
}
