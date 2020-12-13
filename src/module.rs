//! Module interface.

use std::sync::Mutex;

use crate::{
    allocator::RegionAllocator,
    commands,
    commands_bus::CommandsBus,
    commands_bus::Source,
    data::CommandPayload,
    data::{Command, Vec2f},
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

impl ModuleState {
    /// Create a new module state.
    pub fn new() -> Self {
        ModuleState {
            gapi_commands_allocator: Mutex::new(RegionAllocator::new(1024)),
            gapi_commands_data_allocator: Mutex::new(RegionAllocator::new(1024)),
            commands_bus: CommandsBus::new(),
        }
    }
}

/// Client module e.g. iOS application, web application etc.
pub struct ClientModule {}

impl ClientModule {
    /// Create a new client module.
    pub fn new() -> Self {
        ClientModule {}
    }
}

impl Module for ClientModule {
    fn init(&mut self, _: &mut ModuleState) {}

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, _: &mut ModuleState) {}

    fn render(&mut self, _: &mut ModuleState) {}
}

/// Demo module
pub struct BenchmarkModule {}

impl BenchmarkModule {
    /// Create a new benchmark module.
    pub fn new() -> BenchmarkModule {
        BenchmarkModule {}
    }
}

impl Module for BenchmarkModule {
    fn init(&mut self, state: &mut ModuleState) {
        let command = Command::empty(commands::gapi::SET_COLOR_PIPELINE);
        let commands_bus = &state.commands_bus;
        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);
    }

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, _: &mut ModuleState) {}

    fn render(&mut self, state: &mut ModuleState) {
        let commands_bus = &state.commands_bus;

        let points = [
            Vec2f::new(0.0, 0.0),
            Vec2f::new(100.0, 0.0),
            Vec2f::new(100.0, 100.0),
            Vec2f::new(0.0, 100.0),
        ];

        let command_payload = CommandPayload::new(&points);
        let command = Command::new(commands::gapi::DRAW_PATH, command_payload);

        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);
    }
}
