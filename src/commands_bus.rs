//! Commands Bus

use std::mem;

use crate::{STATE, commands::Source, data::{Command, BytesBuffer}};

/// Commands bus. Used to communicate between modules.
pub struct CommandsBus {}

impl Default for CommandsBus {
    fn default() -> Self {
        CommandsBus::new()
    }
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
    /// let payload = unsafe { BytesBuffer::new(&[12, 34, 55]) };
    /// let command = Command::new(commands::gapi::DRAW_LINES, payload);
    /// let commands_bus = CommandsBus::new();
    /// commands_bus.push_command(module::CLIENT_ID, command, commands::Source::GAPI);
    /// ```
    pub fn push_command(&self, address: usize, command: Command, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = unsafe { STATE.as_ref() }.unwrap();
        let module_state = state.module_states.get(address).unwrap();

        let (mut commands_allocator_guard, mut commands_data_allocator_guard) = match source {
            Source::GAPI => {
                (
                    module_state.gapi_commands_allocator.lock(),
                    module_state.gapi_commands_data_allocator.lock(),
                )
            }
            Source::Processor => {
                unimplemented!()
            }
        };

        let commands_allocator = commands_allocator_guard.as_mut().unwrap();
        let commands_data_allocator = commands_data_allocator_guard.as_mut().unwrap();

        let data = unsafe {
            commands_data_allocator
                .emplace_buffer(command.payload.base, command.payload.size)
                .unwrap()
        };

        let command_payload = BytesBuffer {
            base: data,
            size: command.payload.size,
        };
        let command = Command::new(command.id, command_payload);
        commands_allocator.emplace_struct(&command).unwrap();
    }
}
