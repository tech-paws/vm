pub mod draw_centered_quads;
pub mod draw_quads;
pub mod draw_texts;
pub mod set_color_pipeline;
pub mod set_texture_pipeline;

/// Command id to draw lines.
pub const DRAW_LINES: u64 = 0x0002_0001;

/// Command id to draw lines.
pub const DRAW_PATH: u64 = 0x0002_0002;

/// Command id to draw quads.
pub const DRAW_QUADS: u64 = 0x0002_0003;

/// Command id to draw quads with center pivot.
pub const DRAW_CENTERED_QUADS: u64 = 0x0002_0004;

/// Command id to draw texts.
pub const DRAW_TEXTS: u64 = 0x0002_0005;

/// Set current pipelen to colorize.
pub const SET_COLOR_PIPELINE: u64 = 0x0002_0006;

/// Set current pipelen to texture.
pub const SET_TEXTURE_PIPELINE: u64 = 0x0002_0007;
