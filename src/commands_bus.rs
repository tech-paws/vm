//! Commands Bus

use std::{ffi::CStr, os::raw::c_char, ptr::null};

use crate::{
    commands::Source,
    data::{BytesBuffer, CCommand, Command},
    STATE,
};

/// Commands bus. Used to communicate between modules.
pub struct CommandsBus {
    module_id: &'static str,
}

impl CommandsBus {
    /// Create a new commands bus.
    pub fn new(module_id: &'static str) -> Self {
        CommandsBus { module_id }
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
    pub fn push_command(&self, address: &'static str, command: Command, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = unsafe { STATE.as_ref() }.unwrap();
        let module_state = state.module_states.get(&address).unwrap();

        let (
            mut commands_allocator_guard,
            mut commands_data_allocator_guard,
            mut commands_payload_allocator_guard,
        ) = match source {
            Source::GAPI => (
                module_state.gapi_commands_allocator.lock(),
                module_state.gapi_commands_data_allocator.lock(),
                module_state.gapi_commands_payload_allocator.lock(),
            ),
            Source::Processor => unimplemented!(),
        };

        let commands_allocator = commands_allocator_guard.as_mut().unwrap();
        let commands_data_allocator = commands_data_allocator_guard.as_mut().unwrap();
        let commands_payload_allocator = commands_payload_allocator_guard.as_mut().unwrap();

        let mut payload_base: *const BytesBuffer = null::<BytesBuffer>();

        for payload in command.payload {
            let data = unsafe {
                commands_data_allocator
                    .emplace_buffer(payload.base, payload.size)
                    .unwrap()
            };

            let command_payload = BytesBuffer {
                base: data,
                size: payload.size,
            };

            let payload_data = commands_payload_allocator
                .emplace_struct(&command_payload)
                .unwrap();

            if payload_base == null::<BytesBuffer>() {
                payload_base = payload_data;
            }
        }

        let from_address = BytesBuffer {
            base: self.module_id.as_ptr(),
            size: self.module_id.len() as u64,
        };

        let command = CCommand {
            id: command.id,
            count: command.payload.len() as u64,
            from: from_address,
            payload: payload_base,
        };
        commands_allocator.emplace_struct(&command).unwrap();
    }

    pub fn c_push_command(&self, address: *const c_char, command: CCommand, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = unsafe { STATE.as_ref() }.unwrap();
        let address: &str = unsafe { CStr::from_ptr(address) }.to_str().unwrap();

        let module_state = state.module_states.get(&address).unwrap();

        // let mut commands_allocator_guard = match source {
        //     Source::GAPI => module_state.gapi_commands_allocator.lock(),
        //     Source::Processor => module_state.processor_commands_allocator.lock(),
        // };

        // let commands_allocator = commands_allocator_guard.as_mut().unwrap();
        let (
            mut commands_allocator_guard,
            mut commands_data_allocator_guard,
            mut commands_payload_allocator_guard,
        ) = match source {
            Source::GAPI => (
                module_state.gapi_commands_allocator.lock(),
                module_state.gapi_commands_data_allocator.lock(),
                module_state.gapi_commands_payload_allocator.lock(),
            ),
            Source::Processor => (
                module_state.processor_commands_allocator.lock(),
                module_state.processor_commands_data_allocator.lock(),
                module_state.processor_commands_payload_allocator.lock(),
            ),
        };

        let commands_allocator = commands_allocator_guard.as_mut().unwrap();
        let commands_data_allocator = commands_data_allocator_guard.as_mut().unwrap();
        let commands_payload_allocator = commands_payload_allocator_guard.as_mut().unwrap();

        let data = unsafe {
            commands_data_allocator
                .emplace_buffer((*command.payload).base, (*command.payload).size)
                .unwrap()
        };

        let command_payload = BytesBuffer {
            base: data,
            size: unsafe { (*command.payload).size },
        };

        let payload_base = commands_payload_allocator
            .emplace_struct(&command_payload)
            .unwrap();

        let command = CCommand {
            id: command.id,
            count: 1,
            from: command.from,
            payload: payload_base,
        };

        commands_allocator.emplace_struct(&command).unwrap();
    }
}
