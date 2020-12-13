//! Module interface.

use std::sync::Mutex;

use crate::allocator::RegionAllocator;

// TODO(sysint64): Make it dynamic
/// Benchmark module id.
pub const BENCHMARK_ID: usize = 0;

/// Debug services module id.
pub const DEBUG_ID: usize = 1;

/// Module interface.
pub trait Module {
    /// Initialize module, e.g. run process or server
    fn init(&mut self);

    /// Shutdown module, e.g. stop process, or stop server, free resources
    fn shutdown(&mut self);

    /// Progress, put here some computations
    fn step(&mut self);

    /// Rendering
    fn render(&mut self);
}

/// Module state.
pub struct ModuleState {
    /// Rendering commands.
    pub gapi_commands_allocator: Mutex<RegionAllocator>,

    /// Here is a data that holds rendering commands.
    pub gapi_commands_data_allocator: Mutex<RegionAllocator>,
}

impl ModuleState {
    /// Create a new module state.
    pub fn new() -> Self {
        ModuleState {
            gapi_commands_allocator: Mutex::new(RegionAllocator::new(1024)),
            gapi_commands_data_allocator: Mutex::new(RegionAllocator::new(1024)),
        }
    }
}
