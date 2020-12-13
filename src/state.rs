//! Virtual machine state.

use std::sync::Mutex;

use crate::{allocator::RegionAllocator, module::Module};

/// State structure.
pub struct VMState {
    // TODO(sysint64): Improve doc.
    /// Rendering commands.
    pub gapi_commands_allocator: Mutex<RegionAllocator>,

    /// Here is a data that holds rendering commands.
    pub gapi_commands_data_allocator: Mutex<RegionAllocator>,

    /// Connected modules.
    pub modules: Mutex<Vec<Box<dyn Module>>>,
}

impl VMState {
    /// Create a new state.
    pub fn new() -> Self {
        VMState {
            gapi_commands_allocator: Mutex::new(RegionAllocator::new(1024)),
            gapi_commands_data_allocator: Mutex::new(RegionAllocator::new(1024)),
            modules: Mutex::new(Vec::new()),
        }
    }
}
