//! Virtual machine primitive datas.

use std::{mem, ops, ptr::null};

/// Virtual machine command payload.
#[derive(Debug)]
#[repr(C)]
pub struct BytesBuffer {
    /// Size of the data.
    pub size: u64,
    /// Base address of the data.
    pub base: *const u8,
}

/// Virtual machine command.
#[derive(Debug)]
#[repr(C)]
pub struct Command {
    /// Unique id of the command.
    pub id: u64,
    /// The data that holds the command.
    pub payload: BytesBuffer,
}

/// Commands array.
#[repr(C)]
pub struct Commands {
    /// Address to commands region
    pub commands: *const Command,
    /// Count of commands.
    pub size: usize,
}

impl Command {
    /// Create a new command with a given payload.
    pub fn new(id: u64, payload: BytesBuffer) -> Self {
        Command { id, payload }
    }

    /// Create a command without a payload.
    pub fn empty(id: u64) -> Self {
        Command {
            id,
            payload: BytesBuffer::empty(),
        }
    }
}

impl Commands {
    /// Create a commands instance without commands.
    pub fn empty() -> Self {
        Commands {
            size: 0,
            commands: null::<Command>(),
        }
    }
}

// pub enum CommandPayloadItem {
//     Int32(i32),
//     Int64(i64),
//     Vec2f(Vec2f),
//     Vec4f(Vec4f),
// }

// union CommandPayloadItemC {
//     int32: i32,
//     int64: i64,
//     vec2f: Vec2f,
//     vec4f: Vec4f,
// }

// struct CommandPayload {
//     data: Vec<CommandPayloadItemC>,
// }

// impl CommandPayload {
//     pub fn new(items: &[CommandPayloadItem]) -> Self {
//         for item in items.iter() {
//         }

//         todo!()
//     }
// }

impl BytesBuffer {
    /// Create a new payload with a given array.
    pub fn new<T>(values: &[T]) -> Self {
        let base = values
            .first()
            .map(|value| value as *const T)
            .unwrap_or(null::<T>());

        BytesBuffer {
            size: (mem::size_of::<T>() as u64) * (values.len() as u64),
            base: base as *const u8,
        }
    }

    /// Create a new payload with a given array.
    // pub fn new_command_payload(_values: &[CommandPayloadItem]) -> Self {
        // todo!()
    // }

    /// Create empty payload without any data.
    pub fn empty() -> Self {
        BytesBuffer {
            size: 0,
            base: null::<u8>(),
        }
    }
}

/// 2D Vector with float components.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2f {
    /// x component.
    pub x: f32,
    /// y component.
    pub y: f32,
}

/// 4D Vector with float components.
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec4f {
    /// x component.
    pub x: f32,
    /// y component.
    pub y: f32,
    /// z component.
    pub z: f32,
    /// w component.
    pub w: f32,
}

/// 4x4 Matrix with float components.
#[repr(C)]
pub struct Mat4f {
    /// Columns
    pub col: [Vec4f; 4],
}

impl Vec2f {
    /// Const value for zero value: `Vec2::new(0., 0.)`.
    pub const ZERO: Vec2f = Vec2f::new(0., 0.);

    /// Create a new vector.
    pub const fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }
}

impl Vec4f {
    /// Const value for zero value: `Vec4::new(0., 0., 0., 0.)`.
    pub const ZERO: Vec4f = Vec4f::new(0., 0., 0., 0.);

    /// Create a new vector.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f { x, y, z, w }
    }
}

impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
