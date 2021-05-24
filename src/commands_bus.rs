//! Commands Bus

use parking_lot::lock_api::RawMutex;
use std::{ffi::CStr, os::raw::c_char, ptr::null, sync::MutexGuard};
use vm_buffers::{BytesWriter, IntoVMBuffers};
use vm_memory::BufferAccessor;

use crate::{
    commands::Source,
    data::{BytesBuffer, CCommand, Command, MutBytesBuffer},
    STATE,
};

/// Commands bus. Used to communicate between modules.
pub struct CommandsBus {
    module_id: &'static str,
    start_offset: u64,
    payload_size_offset: u64,
}

impl CommandsBus {
    /// Create a new commands bus.
    pub fn new(module_id: &'static str) -> Self {
        CommandsBus {
            module_id,
            start_offset: 0,
            payload_size_offset: 0,
        }
    }

    pub fn push_command_new<F>(
        &self,
        address: &'static str,
        id: u64,
        source: Source,
        command_writer: F,
    ) where
        F: FnOnce(&mut BytesWriter),
    {
        let state = unsafe { STATE.as_ref() }.unwrap();
        let module_state = state.module_states.get(&address).unwrap();

        let (mut bytes_writer, mut bytes_reader) = match source {
            Source::GAPI => {
                (
                    module_state.gapi_bytes_writer.lock(),
                    module_state.gapi_bytes_reader.lock(),
                )
            }
            Source::Processor => {
                (
                    module_state.processor_bytes_writer.lock(),
                    module_state.processor_bytes_reader.lock(),
                )
            }
        };

        // Update commands count
        let commands_count = bytes_reader.read_u64_at(0);

        bytes_writer.write_u64_at(0, commands_count + 1);
        bytes_writer.write_u64(id);

        // Write size of payload in bytes
        let payload_size_offset = bytes_writer.current_offset();
        // Leave gap for the future, because we don't know the actual size of payload yet.
        bytes_writer.write_u64(0);

        let start_offset = bytes_writer.current_offset();
        command_writer(&mut bytes_writer);
        let end_offset = bytes_writer.current_offset();

        // Write size of payload at size_offset
        bytes_writer.write_u64_at(payload_size_offset, end_offset - start_offset);
    }

    /// Get commands from the root module.
    pub unsafe fn begin_command(
        &mut self,
        address: &str,
        source: Source,
        id: u64,
    ) -> *mut vm_buffers::c_api::BytesWriter {
        let state = STATE.as_ref().unwrap();
        let module_state = &mut state.module_states.get(address).unwrap();

        let bytes_writer = match source {
            Source::GAPI => {
                module_state.gapi_bytes_writer.raw().lock();
                module_state.gapi_bytes_writer.data_ptr().as_mut().unwrap()
            }
            Source::Processor => {
                module_state.processor_bytes_writer.raw().lock();
                module_state
                    .processor_bytes_writer
                    .data_ptr()
                    .as_mut()
                    .unwrap()
            }
        };

        bytes_writer.write_u64(id);

        // Write size of payload in bytes
        self.payload_size_offset = bytes_writer.current_offset();
        // Leave gap for the future, because we don't know the actual size of payload yet.
        bytes_writer.write_u64(0);

        self.start_offset = bytes_writer.current_offset();

        bytes_writer.raw()
    }

    pub unsafe fn end_command(&mut self, address: &str, source: Source) {
        let state = STATE.as_ref().unwrap();
        let module_state = &mut state.module_states.get(address).unwrap();

        let mut bytes_reader = match source {
            Source::GAPI => module_state.gapi_bytes_reader.lock(),
            Source::Processor => module_state.processor_bytes_reader.lock(),
        };

        let bytes_writer = match source {
            Source::GAPI => module_state.gapi_bytes_writer.data_ptr().as_mut().unwrap(),
            Source::Processor => {
                module_state
                    .processor_bytes_writer
                    .data_ptr()
                    .as_mut()
                    .unwrap()
            }
        };

        // Update commands count
        let commands_count = bytes_reader.read_u64_at(0);
        bytes_writer.write_u64_at(0, commands_count + 1);

        let end_offset = bytes_writer.current_offset();
        // Write size of payload at size_offset
        bytes_writer.write_u64_at(self.payload_size_offset, end_offset - self.start_offset);

        // unlock mutexes
        match source {
            Source::GAPI => {
                module_state.gapi_commands_allocator_new.raw().unlock();
                module_state.gapi_bytes_writer.raw().unlock();
            }
            Source::Processor => {
                module_state.processor_commands_allocator_new.raw().unlock();
                module_state.processor_bytes_writer.raw().unlock();
            }
        }
    }

    /// Push command to module by address using the allocator `source` to
    /// store commands.
    pub fn push_command(&self, address: &'static str, command: Command, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = unsafe { STATE.as_ref() }.unwrap();
        let module_state = state.module_states.get(&address).unwrap();

        let (mut commands_allocator, mut commands_data_allocator, mut commands_payload_allocator) =
            match source {
                Source::GAPI => {
                    (
                        module_state.gapi_commands_allocator.lock(),
                        module_state.gapi_commands_data_allocator.lock(),
                        module_state.gapi_commands_payload_allocator.lock(),
                    )
                }
                Source::Processor => unimplemented!(),
            };

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

    pub unsafe fn c_push_command(&self, address: *const c_char, command: CCommand, source: Source) {
        // TODO(sysint64): handle unwraps.
        let state = STATE.as_ref().unwrap();
        let address: &str = CStr::from_ptr(address).to_str().unwrap();

        let module_state = state.module_states.get(&address).unwrap();

        // let mut commands_allocator_guard = match source {
        //     Source::GAPI => module_state.gapi_commands_allocator.lock(),
        //     Source::Processor => module_state.processor_commands_allocator.lock(),
        // };

        // let commands_allocator = commands_allocator_guard.as_mut().unwrap();
        let (mut commands_allocator, mut commands_data_allocator, mut commands_payload_allocator) =
            match source {
                Source::GAPI => {
                    (
                        module_state.gapi_commands_allocator.lock(),
                        module_state.gapi_commands_data_allocator.lock(),
                        module_state.gapi_commands_payload_allocator.lock(),
                    )
                }
                Source::Processor => {
                    (
                        module_state.processor_commands_allocator.lock(),
                        module_state.processor_commands_data_allocator.lock(),
                        module_state.processor_commands_payload_allocator.lock(),
                    )
                }
            };

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
