#![warn(missing_docs)]

//! Virtual machine memory management.

pub mod allocator;
pub mod c_api;
pub mod commands;
pub mod data;
pub mod gapi;

use lazy_static::lazy_static;
use std::sync::Mutex;

use allocator::RegionAllocator;
use data::Command;

lazy_static! {
    static ref GAPI_COMMANDS_ALLOCATOR: Mutex<RegionAllocator> =
        Mutex::new(RegionAllocator::new(1024));
}

/// In what allocator put your data
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
}

/// Push command to the allocator `source`.
///
/// # Examples
///
/// ```rust
/// # use assert_approx_eq::assert_approx_eq;
/// use std::mem;
/// use vm::allocator::*;
/// use vm::commands;
/// use vm::data::*;
/// use vm::*;
///
/// let payload = unsafe { CommandPayload::new(&12) };
/// let command = Command::new(commands::gapi::DRAW_LINES, payload);
/// push_command(command, Source::GAPI);
/// ```
#[no_mangle]
pub extern "C" fn push_command(command: Command, source: Source) {
    let mut allocator_guard = match source {
        Source::GAPI => GAPI_COMMANDS_ALLOCATOR.lock(),
    };

    let allocator = allocator_guard.as_mut().unwrap();

    unsafe { allocator.emplace_struct(&command.id) }.unwrap();
    unsafe { allocator.emplace_buffer(command.payload.base, command.payload.size) }.unwrap();
}
