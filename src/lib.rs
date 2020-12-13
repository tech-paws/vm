#![warn(missing_docs)]

//! Virtual machine memory management.

pub mod allocator;
pub mod c_api;
pub mod commands;
pub mod data;
pub mod gapi;
pub mod module;
pub mod state;
pub mod commands_bus;

use state::VMState;

use data::{Command, CommandPayload, Commands};

static mut STATE: Option<VMState> = None;

/// In what allocator put your data
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
}

/// Initialize VM State.
#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE = Some(VMState::new());
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
/// unsafe { vm::init() };
/// let payload = unsafe { CommandPayload::new(&[12, 34, 55]) };
/// let command = Command::new(commands::gapi::DRAW_LINES, payload);
/// unsafe { push_command(command, Source::GAPI) };
/// ```
#[no_mangle]
pub unsafe extern "C" fn push_command(command: Command, source: Source) {
    let state = STATE.as_ref().unwrap();

    let (mut commands_allocator_guard, mut commands_data_allocator_guard) = match source {
        Source::GAPI => (
            state.gapi_commands_allocator.lock(),
            state.gapi_commands_data_allocator.lock(),
        ),
    };

    let commands_allocator = commands_allocator_guard.as_mut().unwrap();
    let commands_data_allocator = commands_data_allocator_guard.as_mut().unwrap();

    let data = commands_data_allocator
        .emplace_buffer(command.payload.base, command.payload.size)
        .unwrap();

    let command_payload = CommandPayload {
        base: data,
        size: command.payload.size,
    };
    let command = Command::new(command.id, command_payload);
    commands_allocator.emplace_struct(&command).unwrap();
}

/// Get all commands from the source.
pub extern "C" fn get_commands(_source: Source) -> Commands {
    todo!()
}

/// Clear all commands from the source.
pub extern "C" fn clear_commands(_source: Source) {
    todo!()
}
