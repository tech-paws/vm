//! Commands Bus

use parking_lot::lock_api::RawMutex;
use vm_buffers::BytesWriter;

use crate::{commands::Source, STATE};

/// Commands bus. Used to communicate between modules.
pub struct CommandsBus {
    start_offset: u64,
    payload_size_offset: u64,
}

impl Default for CommandsBus {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandsBus {
    /// Create a new commands bus.
    pub fn new() -> Self {
        CommandsBus {
            start_offset: 0,
            payload_size_offset: 0,
        }
    }

    /// Send command to address.
    pub fn push_command<F>(
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
                    module_state.gapi_commands.bytes_writer.lock(),
                    module_state.gapi_commands.bytes_reader.lock(),
                )
            }
            Source::Processor => {
                (
                    module_state.processor_commands.bytes_writer.lock(),
                    module_state.processor_commands.bytes_reader.lock(),
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
                module_state.gapi_commands.bytes_writer.raw().lock();
                module_state
                    .gapi_commands
                    .bytes_writer
                    .data_ptr()
                    .as_mut()
                    .unwrap()
            }
            Source::Processor => {
                module_state.processor_commands.bytes_writer.raw().lock();
                module_state
                    .processor_commands
                    .bytes_writer
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
            Source::GAPI => module_state.gapi_commands.bytes_reader.lock(),
            Source::Processor => module_state.processor_commands.bytes_reader.lock(),
        };

        let bytes_writer = match source {
            Source::GAPI => {
                module_state
                    .gapi_commands
                    .bytes_writer
                    .data_ptr()
                    .as_mut()
                    .unwrap()
            }
            Source::Processor => {
                module_state
                    .processor_commands
                    .bytes_writer
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
                module_state.gapi_commands.allocator.raw().unlock();
                module_state.gapi_commands.bytes_writer.raw().unlock();
            }
            Source::Processor => {
                module_state.processor_commands.allocator.raw().unlock();
                module_state.processor_commands.bytes_writer.raw().unlock();
            }
        }
    }
}
