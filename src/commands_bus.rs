//! Commands Bus
//!
//! Module implements abstraction for sending commands to a different modules.

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
    /// Create a new `CommandsBus`.
    pub fn new() -> Self {
        CommandsBus {
            start_offset: 0,
            payload_size_offset: 0,
        }
    }

    /// Send command to the module.
    ///
    /// Forms a command with the `id` and stores it to the `source` buffer
    /// of the module at the `address`;
    /// `commands_writer` is used to write the command payload.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vm::{commands, commands_bus::CommandsBus, module};
    ///
    /// unsafe { vm::init() };
    /// let commands_bus = CommandsBus::new();
    /// commands_bus.push_command(
    ///     module::CLIENT_ID,
    ///     commands::gapi::SET_COLOR_PIPELINE,
    ///     commands::Source::GAPI,
    ///     |bytes_writer| {
    ///         bytes_writer.write_f32(0.0); // r
    ///         bytes_writer.write_f32(1.0); // g
    ///         bytes_writer.write_f32(1.0); // b
    ///         bytes_writer.write_f32(0.5); // a
    ///     },
    /// );
    /// ```
    pub fn push_command<F>(&self, address: &str, id: u64, source: Source, command_writer: F)
    where
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

    /// Start writing command.
    ///
    /// [`CommandsBus::begin_command`] and [`CommandsBus::end_command`] is an unsafe
    /// alternative of [`CommandsBus::push_command`]. Can be useful for FFI.
    /// Returns `bytes_writer` that used to write the command payload.
    ///
    /// # Safety
    ///
    /// * Should always end command.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vm::{commands, commands_bus::CommandsBus, module};
    /// use vm_buffers::BytesWriter;
    ///
    /// unsafe { vm::init() };
    /// let mut commands_bus = CommandsBus::new();
    /// let bytes_writer_raw = unsafe {
    ///     commands_bus.begin_command(
    ///         module::CLIENT_ID,
    ///         commands::Source::GAPI,
    ///         commands::gapi::SET_COLOR_PIPELINE,
    ///     )
    /// };
    /// let mut bytes_writer = unsafe { BytesWriter::from_raw(*bytes_writer_raw) };
    /// bytes_writer.write_f32(0.0); // r
    /// bytes_writer.write_f32(1.0); // g
    /// bytes_writer.write_f32(1.0); // b
    /// bytes_writer.write_f32(0.5); // a
    /// unsafe { commands_bus.end_command(module::CLIENT_ID, commands::Source::GAPI) };
    /// ```
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

    /// Finish writing command and send it to the module at the `address`.
    ///
    /// [`CommandsBus::begin_command`] and [`CommandsBus::end_command`] is an unsafe
    /// alternative of [`CommandsBus::push_command`]. Can be useful for FFI.
    ///
    /// # Safety
    ///
    /// * Should always be used after [`CommandsBus::begin_command`] has been executed.
    /// * `source` should be the same as it was in [`CommandsBus::begin_command`].
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
