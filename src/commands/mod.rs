pub mod gapi;

use vm_buffers::{BytesReader, BytesWriter};

/// Execute macro.
pub const EXECUTE_MACRO: u64 = 0x0001_0001;

/// Start recording macro.
pub const BEGIN_MACRO: u64 = 0x0001_0002;

/// End recording macro.
pub const END_MACRO: u64 = 0x0001_0003;

pub const UPDATE_VIEWPORT: u64 = 0x0001_0004;

pub const ADD_TEXT_BOUNDARIES: u64 = 0x0001_0005;

/// In what allocator put your data
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
    /// PROCESSOR
    Processor = 1,
}

pub trait CommandNew<T> {
    fn id() -> u64;

    fn source() -> Source;

    fn write(&self, bytes_writer: &mut BytesWriter);

    fn read(bytes_reader: &mut BytesReader) -> T;
}
