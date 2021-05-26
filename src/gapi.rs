//! Graphical API.
//!
//! The `gapi` module is the abstracted interface for sending
//! render commands.
//!
//! # Examples
//!
//! ```rust
//! use vm::{commands, commands_bus::CommandsBus, gapi, module};
//! use vm_math::*;
//!
//! unsafe { vm::init() };
//! let mut commands_bus = CommandsBus::new();
//!
//! let gapi_context = gapi::GApiContext {
//!     from: "my_module_id",
//!     address: module::CLIENT_ID,
//!     commands_bus: &mut commands_bus,
//! };
//!
//! // Used just for the example
//! let quad1_mvp_matrix = Mat4f::IDENT;
//! let quad2_mvp_matrix = Mat4f::IDENT;
//! let text_mvp_matrix = Mat4f::IDENT;
//!
//! gapi::set_color_pipeline(&gapi_context, Vec4f::new(1.0, 1.0, 0.0, 1.0));
//! gapi::draw_centered_quads(&gapi_context, &[quad1_mvp_matrix, quad2_mvp_matrix]);
//!
//! // To render text, we should apply font texture
//! gapi::set_texture_pipeline(&gapi_context, 0);
//!
//! let text_data = gapi::TextData {
//!     font_id: 0,
//!     font_size: 20,
//!     mvp_matrix: text_mvp_matrix,
//!     text: String::from("Hello World!"),
//! };
//!
//! gapi::draw_texts(&gapi_context, &[text_data]);
//! ```
//!
//! To get more examples check out vm_benchmarks.

use vm_buffers::{BytesReader, BytesWriter, IntoVMBuffers};
use vm_math::{Mat4f, Vec2f, Vec4f};

use crate::{commands, commands_bus::CommandsBus};

/// Data to render text.
pub struct TextData {
    /// Asset id for the font to be used to render the text.
    pub font_id: u64,

    /// Font size to be used to render the text.
    pub font_size: u32,

    /// Transformation matrix of the text.
    pub mvp_matrix: Mat4f,

    /// The text to be rendered.
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

/// Graphical API context, holds important information to render
pub struct GApiContext<'a> {
    /// The address where to send the render commands.
    pub address: &'static str,

    /// Commands sender address.
    pub from: &'static str,

    /// Commands bus used to send commands.
    pub commands_bus: &'a mut CommandsBus,
}

/// Set current pipeline as color - a shader will be used that colorizes
/// objects with the `color`.
pub fn set_color_pipeline(context: &GApiContext, color: Vec4f) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::SET_COLOR_PIPELINE,
        commands::Source::GAPI,
        |bytes_writer| {
            color.write_to_buffers(bytes_writer);
        },
    );
}

/// Set current pipeline as texture - a shader will be used that applies
/// texture to objects. The texture will be obtained by asset id = `id`.
pub fn set_texture_pipeline(context: &GApiContext, id: u64) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::SET_TEXTURE_PIPELINE,
        commands::Source::GAPI,
        |bytes_writer| {
            bytes_writer.write_u64(id);
        },
    );
}

/// Render a group of quads with a given `mvp_matrices`
/// that is applied to the quads to display on the screen.
///
/// All quads have a center pivot point.
pub fn draw_centered_quads(context: &GApiContext, mvp_matrices: &[Mat4f]) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::DRAW_CENTERED_QUADS,
        commands::Source::GAPI,
        |bytes_writer| {
            bytes_writer.write_u64(mvp_matrices.len() as u64);

            for mat in mvp_matrices.iter() {
                mat.write_to_buffers(bytes_writer);
            }
        },
    );
}

/// Render a group of quads with a given `mvp_matrices`
/// that is applied to the quads to display on the screen.
///
/// All quads have a pivot point in the upper left corner.
pub fn draw_quads(context: &GApiContext, mvp_matrices: &[Mat4f]) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::DRAW_QUADS,
        commands::Source::GAPI,
        |bytes_writer| {
            bytes_writer.write_u64(mvp_matrices.len() as u64);

            for mat in mvp_matrices.iter() {
                mat.write_to_buffers(bytes_writer);
            }
        },
    );
}

/// Render a group of lines with a given `mvp_matrix`
/// that is applied to group of lines to display on the screen.
///
/// Every two point in `points` describe a line.
///
/// Group has a pivot point in the upper left corner.
pub fn draw_lines(context: &GApiContext, mvp_matrix: &Mat4f, points: &[Vec2f]) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::DRAW_LINES,
        commands::Source::GAPI,
        |bytes_writer| {
            mvp_matrix.write_to_buffers(bytes_writer);
            bytes_writer.write_u64(points.len() as u64);

            for point in points.iter() {
                point.write_to_buffers(bytes_writer);
            }
        },
    );
}

/// Render a group of connected straight lines with a given `mvp_matrix`
/// that is applied to group of lines to display on the screen.
///
/// Every line connects with a previous point.
///
/// Group has a pivot point in the upper left corner.
pub fn draw_path(context: &GApiContext, mvp_matrix: &Mat4f, points: &[Vec2f]) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::DRAW_PATH,
        commands::Source::GAPI,
        |bytes_writer| {
            mvp_matrix.write_to_buffers(bytes_writer);
            bytes_writer.write_u64(points.len() as u64);

            for point in points.iter() {
                point.write_to_buffers(bytes_writer);
            }
        },
    );
}

/// Render a group of texts with a given `texts` that describe
/// the properties of the texts.
pub fn draw_texts(context: &GApiContext, texts: &[TextData]) {
    context.commands_bus.push_command(
        context.address,
        commands::gapi::DRAW_TEXTS,
        commands::Source::GAPI,
        |bytes_writer| {
            context.from.to_string().write_to_buffers(bytes_writer);
            bytes_writer.write_u64(texts.len() as u64);

            for text in texts.iter() {
                text.write_to_buffers(bytes_writer);
            }
        },
    );
}
