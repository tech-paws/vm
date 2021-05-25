//! Module interface.

use std::time::Instant;

use parking_lot::Mutex;
use vm_buffers::{ByteOrder, BytesReader, BytesWriter, IntoVMBuffers};
use vm_memory::RegionAllocator;

use crate::{commands::Source, commands_bus::CommandsBus, commands_reader::CommandsReader};

/// Debug services module id.
pub const CLIENT_ID: &str = "tech.paws.client";

/// Module interface.
pub trait Module {
    /// Unique module ID
    fn id(&self) -> &'static str;

    /// Initialize module, e.g. run process or server
    fn init(&mut self, state: &mut ModuleState);

    /// Shutdown module, e.g. stop process, or stop server, free resources
    fn shutdown(&mut self, state: &mut ModuleState);

    /// Progress, put here some computations
    fn step(&mut self, state: &mut ModuleState);

    /// Rendering
    fn render(&mut self, state: &mut ModuleState);
}

pub struct ModuleCommands {
    /// Rendering commands.
    pub allocator: Mutex<RegionAllocator>,

    pub bytes_writer: Mutex<BytesWriter>,

    pub bytes_reader: Mutex<BytesReader>,
}

impl ModuleCommands {
    pub fn new(module_id: &'static str, capacity: usize) -> Self {
        let allocator = RegionAllocator::new(capacity);
        let mut bytes_writer = BytesWriter::new(ByteOrder::LittleEndian, &allocator);
        let bytes_reader = BytesReader::new(ByteOrder::LittleEndian, &allocator);

        bytes_writer.write_u64(0); // Commands count
        module_id.to_string().write_to_buffers(&mut bytes_writer);

        ModuleCommands {
            allocator: Mutex::new(allocator),
            bytes_writer: Mutex::new(bytes_writer),
            bytes_reader: Mutex::new(bytes_reader),
        }
    }
}

/// Module state.
pub struct ModuleState {
    pub id: String,

    pub text_boundaries_allocator: Mutex<RegionAllocator>,

    pub gapi_commands: ModuleCommands,

    pub processor_commands: ModuleCommands,

    /// Commands bus to communicate with other modules.
    pub commands_bus: CommandsBus,

    ///
    pub last_time: Instant,

    pub delta_time: f32,

    pub last_time_initialized: bool,
}

impl ModuleState {
    /// Create a new module state.
    pub fn new(module_id: &'static str) -> Self {
        ModuleState {
            id: module_id.to_string(),
            text_boundaries_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            gapi_commands: ModuleCommands::new(module_id, 1024),
            processor_commands: ModuleCommands::new(module_id, 1024),
            commands_bus: CommandsBus::new(),
            last_time: Instant::now(),
            delta_time: 0.,
            last_time_initialized: false,
        }
    }

    pub fn get_commands_new<F>(&mut self, source: Source, commands_reader_callback: F)
    where
        F: FnOnce(&mut CommandsReader),
    {
        let mut bytes_reader = match source {
            Source::GAPI => self.gapi_commands.bytes_reader.lock(),
            Source::Processor => self.processor_commands.bytes_reader.lock(),
        };

        // let commands_allocator = match source {
        //     Source::GAPI => self.gapi_commands.allocator.lock(),
        //     Source::Processor => self.processor_commands.allocator.lock(),
        // };

        // if source == Source::Processor {
        //     println!(
        //         "Dump: -------------------------------------------------------------------------"
        //     );

        //     let bytes = unsafe {
        //         std::slice::from_raw_parts(
        //             commands_allocator_new.get_buffer_ptr(),
        //             commands_allocator_new.get_buffer_size() as usize,
        //         )
        //     };
        //     hexdump::hexdump(bytes);
        // }

        let mut commands_reader = CommandsReader::new(&mut bytes_reader);
        commands_reader_callback(&mut commands_reader);
    }

    /// Clear all commands and ther data from source.
    pub fn clear_commands(&mut self, source: Source) -> Result<(), &'static str> {
        let (mut commands_allocator, mut commands_bytes_writer, mut commands_bytes_reader) =
            match source {
                Source::GAPI => {
                    (
                        self.gapi_commands.allocator.lock(),
                        self.gapi_commands.bytes_writer.lock(),
                        self.gapi_commands.bytes_reader.lock(),
                    )
                }
                Source::Processor => {
                    (
                        self.processor_commands.allocator.lock(),
                        self.processor_commands.bytes_writer.lock(),
                        self.processor_commands.bytes_reader.lock(),
                    )
                }
            };

        commands_allocator.clear()?;
        commands_bytes_writer.clear();
        commands_bytes_reader.reset();

        // Write current commands count
        commands_bytes_writer.write_u64(0);
        self.id
            .to_string()
            .write_to_buffers(&mut commands_bytes_writer);

        Ok(())
    }

    pub fn clear_text_boundaries(&mut self) -> Result<(), &'static str> {
        self.text_boundaries_allocator.lock().clear()
    }
}

/// Demo module
pub struct ClientModule {}

impl Default for ClientModule {
    fn default() -> Self {
        ClientModule::new()
    }
}

impl ClientModule {
    /// Create a new benchmark module.
    pub fn new() -> ClientModule {
        ClientModule {}
    }
}

impl Module for ClientModule {
    fn id(&self) -> &'static str {
        CLIENT_ID
    }

    fn init(&mut self, _: &mut ModuleState) {}

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, _: &mut ModuleState) {}

    fn render(&mut self, _: &mut ModuleState) {}
}
