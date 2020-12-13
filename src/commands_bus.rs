//! Commands Bus

use crate::{STATE, data::{Command, CommandPayload}};

/// Commands bus. Used to communicate between modules.
pub struct CommandsBus {}

/// In what allocator put your data
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
}

impl CommandsBus {
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
    /// use vm::module;
    ///
    /// unsafe { vm::init() };
    /// let payload = unsafe { CommandPayload::new(&[12, 34, 55]) };
    /// let command = Command::new(commands::gapi::DRAW_LINES, payload);
    /// unsafe { push_command(command, Source::GAPI) };
    /// ```
    pub unsafe fn push_command(address: usize, command: Command, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = STATE.as_ref().unwrap();
        let mut module_states_guard = state.module_states.lock();
        let module_states = module_states_guard.as_mut().unwrap();
        let module_state = module_states.get(address).unwrap();

        let (mut commands_allocator_guard, mut commands_data_allocator_guard) = match source {
            Source::GAPI => (
                module_state.gapi_commands_allocator.lock(),
                module_state.gapi_commands_data_allocator.lock(),
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
}
