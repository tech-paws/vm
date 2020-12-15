//! Virtual machine state.

use std::sync::Mutex;

use crate::module::{Module, ModuleState};

/// State structure.
pub struct VMState {
    /// Connected modules.
    pub modules: Mutex<Vec<Box<dyn Module>>>,

    /// Module states.
    pub module_states: Mutex<Vec<ModuleState>>,
}

impl Default for VMState {
    fn default() -> Self {
        VMState::new()
    }
}

impl VMState {
    /// Create a new state.
    pub fn new() -> Self {
        VMState {
            modules: Mutex::new(Vec::new()),
            module_states: Mutex::new(Vec::new()),
        }
    }

    /// Register module in the virtual machine.
    pub fn register_module(&mut self, module: Box<dyn Module>) {
        let mut modules_guard = self.modules.lock();
        let mut modules_states_guard = self.module_states.lock();

        let modules = modules_guard.as_mut().unwrap();
        let modules_states = modules_states_guard.as_mut().unwrap();

        assert!(modules.len() == modules_states.len());

        modules.push(module);
        modules_states.push(ModuleState::new());
    }
}
