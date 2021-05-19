//! GAPI

use vm_math::{Mat4f, Vec4f};

use crate::{commands::gapi::draw_texts::TextData, commands_bus::CommandsBus};

pub struct GApiContext<'a> {
    pub address: &'static str,
    pub commands_bus: &'a CommandsBus,
}

pub fn set_color_pipeline(context: &mut GApiContext, color: Vec4f) {}

pub fn set_texture_pipeline(context: &mut GApiContext, id: u64) {}

pub fn draw_centered_quads(context: &mut GApiContext, mvp_matrices: &[Mat4f]) {}

pub fn draw_quads(context: &mut GApiContext, mvp_matrices: &[Mat4f]) {}

pub fn draw_texts(context: &mut GApiContext, texts: &[TextData]) {}
