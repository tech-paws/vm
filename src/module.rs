//! Module interface.

use std::{mem, ptr::null, sync::Mutex};

use crate::{
    allocator::RegionAllocator,
    commands::Source,
    commands_bus::CommandsBus,
    data::{Command, Commands},
};

// TODO(sysint64): Make it dynamic
/// Debug services module id.
pub const CLIENT_ID: usize = 0;

/// Benchmark module id.
pub const BENCHMARK_ID: usize = 1;

/// Debug services module id.
pub const DEBUG_ID: usize = 2;

/// Module interface.
pub trait Module {
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
    /// Rendering commands.
    pub gapi_commands_allocator: Mutex<RegionAllocator>,

    /// Here is a data that holds rendering commands.
    pub gapi_commands_data_allocator: Mutex<RegionAllocator>,

    /// Commands bus to communicate with other modules.
    pub commands_bus: CommandsBus,
}

impl Default for ModuleState {
    fn default() -> Self {
        ModuleState::new()
    }
}

impl ModuleState {
    /// Create a new module state.
    pub fn new() -> Self {
        ModuleState {
            gapi_commands_allocator: Mutex::new(RegionAllocator::new(1024)),
            gapi_commands_data_allocator: Mutex::new(RegionAllocator::new(1024)),
            commands_bus: CommandsBus::new(),
        }
    }

    /// Get commands from source.
    pub fn get_commands(&mut self, source: Source) -> Commands {
        let mut commands_allocator_guard = match source {
            Source::GAPI => self.gapi_commands_allocator.lock(),
            Source::Processor => unimplemented!(),
        };

        let commands_allocator = commands_allocator_guard.as_mut().unwrap();

        Commands {
            size: commands_allocator.region.offset as usize,
            commands: commands_allocator.region.base as *mut Command,
        }
    }

    /// Clear all commands and ther data from source.
    pub fn clear_commands(&mut self, source: Source) -> Result<(), &'static str> {
        let (mut commands_allocator_guard, mut commands_data_allocator_guard) = match source {
            Source::GAPI => (
                self.gapi_commands_allocator.try_lock(),
                self.gapi_commands_data_allocator.try_lock(),
            ),
            Source::Processor => return Ok(())
        };

        let commands_allocator = commands_allocator_guard.as_mut().unwrap();
        let commands_data_allocator = commands_data_allocator_guard.as_mut().unwrap();

        commands_allocator.clear()?;
        commands_data_allocator.clear()?;

        Ok(())
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
    fn init(&mut self, _: &mut ModuleState) {}

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, _: &mut ModuleState) {}

    fn render(&mut self, _: &mut ModuleState) {}
}
