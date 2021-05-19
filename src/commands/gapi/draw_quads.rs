use crate::commands::gapi;
use crate::commands::{CommandNew, Source};
use vm_buffers::{BytesReader, BytesWriter, IntoVMBuffers};
use vm_math::Mat4f;

pub struct Command<'a> {
    pub mvp_matrices: &'a [Mat4f],
}

impl<'a> Command<'a> {
    pub fn new(mvp_matrices: &'a [Mat4f]) -> Self {
        Self { mvp_matrices }
    }
}

impl<'a> CommandNew<Mat4f> for Command<'a> {
    fn id() -> u64 {
        gapi::DRAW_QUADS
    }

    fn source() -> Source {
        Source::GAPI
    }

    fn write(&self, bytes_writer: &mut BytesWriter) {
        for mat in self.mvp_matrices.iter() {
            mat.write_to_buffers(bytes_writer);
        }
    }

    fn read(bytes_reader: &mut BytesReader) -> Mat4f {
         Mat4f::read_from_buffers(bytes_reader)
    }
}
