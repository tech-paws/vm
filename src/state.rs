//! Virtual machine state.

use std::sync::Mutex;

use crate::module::{Module, ModuleState};

/// State structure.
pub struct VMState {
    /// Connected modules.
    pub modules: Mutex<Vec<Box<dyn Module>>>,

    /// Module states.
    pub module_states: Mutex<Vec<Box<ModuleState>>>,
}

impl VMState {
    /// Create a new state.
    pub fn new() -> Self {
        VMState {
            modules: Mutex::new(Vec::new()),
            module_states: Mutex::new(Vec::new()),
        }
    }
}
