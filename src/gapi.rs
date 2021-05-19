//! GAPI

use vm_buffers::{BytesReader, BytesWriter, IntoVMBuffers};
use vm_math::{Mat4f, Vec4f};

use crate::{commands, commands_bus::CommandsBus};

pub struct TextData {
    pub font_id: u64,
    pub font_size: u32,
    pub mvp_matrix: Mat4f,
    pub text: String,
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

pub struct GApiContext<'a> {
    pub address: &'static str,
    pub commands_bus: &'a mut CommandsBus,
}

pub fn set_color_pipeline(context: &GApiContext, color: Vec4f) {
    context.commands_bus.push_command_new(
        context.address,
        commands::gapi::SET_COLOR_PIPELINE,
        commands::Source::GAPI,
        |bytes_writer| {
            color.write_to_buffers(bytes_writer);
        },
    );
}

pub fn set_texture_pipeline(context: &GApiContext, id: u64) {
    context.commands_bus.push_command_new(
        context.address,
        commands::gapi::SET_TEXTURE_PIPELINE,
        commands::Source::GAPI,
        |bytes_writer| {
            bytes_writer.write_u64(id);
        },
    );
}

pub fn draw_centered_quads(context: &GApiContext, mvp_matrices: &[Mat4f]) {
    context.commands_bus.push_command_new(
        context.address,
        commands::gapi::DRAW_CENTERED_QUADS,
        commands::Source::GAPI,
        |bytes_writer| {
            for mat in mvp_matrices.iter() {
                mat.write_to_buffers(bytes_writer);
            }
        },
    );
}

pub fn draw_quads(context: &GApiContext, mvp_matrices: &[Mat4f]) {
    context.commands_bus.push_command_new(
        context.address,
        commands::gapi::DRAW_QUADS,
        commands::Source::GAPI,
        |bytes_writer| {
            for mat in mvp_matrices.iter() {
                mat.write_to_buffers(bytes_writer);
            }
        },
    );
}

pub fn draw_texts(context: &GApiContext, texts: &[TextData]) {
    context.commands_bus.push_command_new(
        context.address,
        commands::gapi::DRAW_TEXTS,
        commands::Source::GAPI,
        |bytes_writer| {
            for text in texts.iter() {
                text.write_to_buffers(bytes_writer);
            }
        },
    );
}
