use crate::commands::gapi;
use crate::commands::{CommandNew, Source};
use vm_buffers::{BytesReader, BytesWriter, IntoVMBuffers};
use vm_math::Mat4f;

pub struct TextData {
    font_id: u64,
    font_size: u32,
    mvp_matrix: Mat4f,
    text: String,
}

impl IntoVMBuffers for TextData {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            font_id: bytes_reader.read_u64(),
            font_size: bytes_reader.read_u32(),
            mvp_matrix: Mat4f::read_from_buffers(bytes_reader),
            text: String::read_from_buffers(bytes_reader),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_u64(self.font_id);
        bytes_writer.write_u32(self.font_size);
        self.mvp_matrix.write_to_buffers(bytes_writer);
        self.text.write_to_buffers(bytes_writer);
    }
}

pub struct Command<'a> {
    pub texts: &'a [TextData],
}

impl<'a> Command<'a> {
    pub fn new(texts: &'a [TextData]) -> Self {
        Self { texts }
    }
}

impl<'a> CommandNew<TextData> for Command<'a> {
    fn id() -> u64 {
        gapi::DRAW_QUADS
    }

    fn source() -> Source {
        Source::GAPI
    }

    fn write(&self, bytes_writer: &mut BytesWriter) {
        for text in self.texts.iter() {
            text.write_to_buffers(bytes_writer);
        }
    }

    fn read(bytes_reader: &mut BytesReader) -> TextData {
        TextData::read_from_buffers(bytes_reader)
    }
}
