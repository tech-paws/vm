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
use data::{Command, CommandPayload};

lazy_static! {
    static ref GAPI_COMMANDS_ALLOCATOR: Mutex<RegionAllocator> =
        Mutex::new(RegionAllocator::new(1024));
    static ref GAPI_COMMANDS_DATA_ALLOCATOR: Mutex<RegionAllocator> =
        Mutex::new(RegionAllocator::new(1024));
}

pub trait Module {
    fn init();

    fn drop();

    fn step();

    fn render();
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
/// let payload = unsafe { CommandPayload::new(&[12, 34, 55]) };
/// let command = Command::new(commands::gapi::DRAW_LINES, payload);
/// push_command(command, Source::GAPI);
/// ```
#[no_mangle]
pub extern "C" fn push_command(command: Command, source: Source) {
    let (mut commands_allocator_guard, mut commands_data_allocator_guard) = match source {
        Source::GAPI => (
            GAPI_COMMANDS_ALLOCATOR.lock(),
            GAPI_COMMANDS_DATA_ALLOCATOR.lock(),
        ),
    };

    let commands_allocator = commands_allocator_guard.as_mut().unwrap();
    let commands_data_allocator = commands_data_allocator_guard.as_mut().unwrap();

    let data = unsafe {
        commands_data_allocator
            .emplace_buffer(command.payload.base, command.payload.size)
            .unwrap()
    };

    let command_payload = CommandPayload {
        base: data,
        size: command.payload.size,
    };
    let command = Command::new(command.id, command_payload);
    unsafe { commands_allocator.emplace_struct(&command) }.unwrap();
}
