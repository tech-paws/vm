//! Module interface.

use std::time::Instant;

use parking_lot::Mutex;
use vm_buffers::{ByteOrder, BytesReader, BytesWriter, IntoVMBuffers};
use vm_memory::{BufferAccessor, RegionAllocator};

use crate::{
    commands::Source,
    commands_bus::CommandsBus,
    data::{CCommand, Commands},
};

// TODO(sysint64): Make it dynamic
/// Debug services module id.
pub const CLIENT_ID: &'static str = "tech.paws.client";

/// Module interface.
pub trait Module {
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

/// Module state.
pub struct ModuleState {
    pub id: String,

    pub text_boundaries_allocator: Mutex<RegionAllocator>,

    /// Rendering commands.
    pub gapi_commands_allocator: Mutex<RegionAllocator>,

    /// Here is a data that holds rendering commands.
    pub gapi_commands_allocator_new: Mutex<RegionAllocator>,

    pub gapi_bytes_writer: Mutex<BytesWriter>,

    pub gapi_bytes_reader: Mutex<BytesReader>,

    pub processor_commands_allocator_new: Mutex<RegionAllocator>,

    pub processor_bytes_writer: Mutex<BytesWriter>,

    pub processor_bytes_reader: Mutex<BytesReader>,

    /// Here is a data that holds rendering commands.
    pub gapi_commands_data_allocator: Mutex<RegionAllocator>,

    pub gapi_commands_payload_allocator: Mutex<RegionAllocator>,

    /// Rendering commands.
    pub processor_commands_allocator: Mutex<RegionAllocator>,

    /// Here is a data that holds rendering commands.
    pub processor_commands_data_allocator: Mutex<RegionAllocator>,

    pub processor_commands_payload_allocator: Mutex<RegionAllocator>,

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
        let gapi_commands_allocator_new = RegionAllocator::new(1024);
        let mut gapi_bytes_writer =
            BytesWriter::new(ByteOrder::LittleEndian, &gapi_commands_allocator_new);
        let gapi_bytes_reader =
            BytesReader::new(ByteOrder::LittleEndian, &gapi_commands_allocator_new);

        // Write current commands count
        gapi_bytes_writer.write_u64(0);
        module_id
            .to_string()
            .write_to_buffers(&mut gapi_bytes_writer);

        //
        let processor_commands_allocator_new = RegionAllocator::new(512);
        let mut processor_bytes_writer =
            BytesWriter::new(ByteOrder::LittleEndian, &processor_commands_allocator_new);
        let processor_bytes_reader =
            BytesReader::new(ByteOrder::LittleEndian, &processor_commands_allocator_new);

        // Write current commands count
        processor_bytes_writer.write_u64(0);
        module_id
            .to_string()
            .write_to_buffers(&mut processor_bytes_writer);

        ModuleState {
            id: module_id.to_string(),
            text_boundaries_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            gapi_commands_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            gapi_bytes_writer: Mutex::new(gapi_bytes_writer),
            gapi_bytes_reader: Mutex::new(gapi_bytes_reader),
            gapi_commands_allocator_new: Mutex::new(gapi_commands_allocator_new),
            processor_bytes_writer: Mutex::new(processor_bytes_writer),
            processor_bytes_reader: Mutex::new(processor_bytes_reader),
            processor_commands_allocator_new: Mutex::new(processor_commands_allocator_new),
            gapi_commands_data_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            gapi_commands_payload_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            processor_commands_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            processor_commands_data_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            processor_commands_payload_allocator: Mutex::new(RegionAllocator::new(1024 * 1024)),
            commands_bus: CommandsBus::new(module_id),
            last_time: Instant::now(),
            delta_time: 0.,
            last_time_initialized: false,
        }
    }

    /// Get commands from source.
    /// TODO(sysint64): Use custom allocator instead of Vec.
    pub fn get_commands(&mut self, source: Source) -> Commands {
        let mut commands_allocator = match source {
            Source::GAPI => self.gapi_commands_allocator.lock(),
            Source::Processor => self.processor_commands_allocator.lock(),
        };

        Commands {
            size: commands_allocator.region.offset as usize,
            commands: commands_allocator.region.base as *mut CCommand,
        }
    }

    /// Clear all commands and ther data from source.
    pub fn clear_commands(&mut self, source: Source) -> Result<(), &'static str> {
        let (
            mut commands_allocator,
            mut commands_allocator_new,
            mut commands_bytes_writer,
            mut commands_bytes_reader,
            mut commands_data_allocator,
            mut commands_payload_allocator,
        ) = match source {
            Source::GAPI => {
                (
                    self.gapi_commands_allocator.lock(),
                    self.gapi_commands_allocator_new.lock(),
                    self.gapi_bytes_writer.lock(),
                    self.gapi_bytes_reader.lock(),
                    self.gapi_commands_data_allocator.lock(),
                    self.gapi_commands_payload_allocator.lock(),
                )
            }
            Source::Processor => {
                (
                    self.processor_commands_allocator.lock(),
                    self.processor_commands_allocator_new.lock(),
                    self.processor_bytes_writer.lock(),
                    self.processor_bytes_reader.lock(),
                    self.processor_commands_data_allocator.lock(),
                    self.processor_commands_payload_allocator.lock(),
                )
            }
        };

        if source == Source::Processor {
            println!(
                "Dump: -------------------------------------------------------------------------"
            );

            let bytes = unsafe {
                std::slice::from_raw_parts(
                    commands_allocator_new.get_buffer_ptr(),
                    commands_allocator_new.get_buffer_size() as usize,
                )
            };
            hexdump::hexdump(bytes);
        }

        commands_allocator.clear()?;
        commands_allocator_new.clear()?;
        commands_data_allocator.clear()?;
        commands_payload_allocator.clear()?;
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
