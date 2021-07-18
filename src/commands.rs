//! Available commands

// TODO(sysint64): Write proper documentation for each of the commands.

/// In what allocator put your data
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
    /// PROCESSOR
    Processor = 1,
}

/// Execute macro.
pub const EXECUTE_MACRO: u64 = 0x0001_0001;

/// Start recording macro.
pub const BEGIN_MACRO: u64 = 0x0001_0002;

/// End recording macro.
pub const END_MACRO: u64 = 0x0001_0003;

/// Update current viewport size.
pub const UPDATE_VIEWPORT: u64 = 0x0001_0004;

/// Add text boundaries.
pub const ADD_TEXT_BOUNDARIES: u64 = 0x0001_0005;

/// On touch start event.
pub const COMMAND_TOUCH_START: u64 = 0x0001_0006;

/// On touch start event.
pub const COMMAND_TOUCH_END: u64 = 0x0001_0007;

/// On touch start event.
pub const COMMAND_TOUCH_MOVE: u64 = 0x0001_0008;

pub const COMMAND_MOUSE_BUTTON_UNKNOWN: u8 = 0;
pub const COMMAND_MOUSE_BUTTON_LEFT: u8 = 1;
pub const COMMAND_MOUSE_BUTTON_RIGHT: u8 = 2;
pub const COMMAND_MOUSE_BUTTON_MIDDLE: u8 = 3;

/// Graphical API commands
pub mod gapi {
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

    /// Set view port.
    pub const SET_VIEWPORT: u64 = 0x0002_0008;
}

/// Transforms commands.
pub mod transforms {
    /// Move.
    pub const TRANSLATE: u64 = 0x0003_0001;

    /// Rotate.
    pub const ROTATE: u64 = 0x0003_0002;

    /// Scale.
    pub const SCALE: u64 = 0x0003_0003;
}

/// Commands to manage assets.
pub mod assets {
    /// Load texture and save in memory.
    pub const LOAD_TEXTURE: u64 = 0x0004_0001;

    /// Load macro and save in memory.
    pub const LOAD_MACRO: u64 = 0x0004_0002;

    /// Remove texture from memory.
    pub const REMOVE_TEXTURE: u64 = 0x0004_0003;

    /// Remove macro from memory.
    pub const REMOVE_MACRO: u64 = 0x0004_0004;
}

/// State commands.
pub mod state {
    /// Update current viewport size.
    pub const UPDATE_VIEW_PORT: u64 = 0x0005_0001;

    /// Update current touch state.
    pub const UPDATE_TOUCH_STATE: u64 = 0x0005_0002;
}
