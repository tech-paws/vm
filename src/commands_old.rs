//! Available commands

// TODO(sysint64): Write proper documentation for each of the commands.

/// Graphical API commands.
pub mod gapi {
    use super::{CommandNew, Source};

    pub struct DrawCenteredQuadsCommand<'a> {
        pub matrices: &'a [Mat4f],
    }

    pub struct DrawQuadsCommand<'a> {
        pub matrices: &'a [Mat4f],
    }



    pub struct DrawTextsCommand<'a> {
        pub texts: &'a [TextData],
    }



    impl SetTexturePipelineCommand {
        fn new(id: u64) -> Self {
            Self { id }
        }
    }


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
