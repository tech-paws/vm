//! Commands Bus

use crate::{
    data::{Command, CommandPayload},
    STATE,
};

/// Commands bus. Used to communicate between modules.
pub struct CommandsBus {}

/// In what allocator put your data
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
}

impl CommandsBus {
    /// Create a new commands bus.
    pub fn new() -> Self {
        CommandsBus {}
    }

    /// Push command to module by address using the allocator `source` to
    /// store commands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use assert_approx_eq::assert_approx_eq;
    /// use std::mem;
    /// use vm::allocator::*;
    /// use vm::commands;
    /// use vm::commands_bus::*;
    /// use vm::data::*;
    /// use vm::module;
    /// use vm::*;
    ///
    /// unsafe { vm::init() };
    /// let payload = unsafe { CommandPayload::new(&[12, 34, 55]) };
    /// let command = Command::new(commands::gapi::DRAW_LINES, payload);
    /// let commands_bus = CommandsBus::new();
    /// commands_bus.push_command(module::CLIENT_ID, command, Source::GAPI);
    /// ```
    pub fn push_command(&self, address: usize, command: Command, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = unsafe { STATE.as_ref() }.unwrap();
        let mut module_states_guard = state.module_states.lock();
        let module_states = module_states_guard.as_mut().unwrap();
        let module_state = module_states.get(address).unwrap();

        let (mut commands_allocator_guard, mut commands_data_allocator_guard) = match source {
            Source::GAPI => {
                (
                    module_state.gapi_commands_allocator.lock(),
                    module_state.gapi_commands_data_allocator.lock(),
                )
            }
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
}
