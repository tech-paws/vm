//! Virtual machine state.

use crate::module::{Module, ModuleState};
use crate::{commands::Source, data::Commands, module};

/// State structure.
pub struct VMState {
    /// Connected modules.
    pub modules: Vec<Box<dyn Module>>,

    /// Module states.
    pub module_states: Vec<ModuleState>,
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
            modules: Vec::new(),
            module_states: Vec::new(),
        }
    }

    /// Register module in the virtual machine.
    pub fn register_module(&mut self, module: Box<dyn Module>) {
        assert!(self.modules.len() == self.module_states.len());

        self.modules.push(module);
        self.module_states.push(ModuleState::new());
    }

    ///
    pub fn step() {}

    ///
    pub fn render() {}

    /// Get commands from the root module.
    pub fn get_commands(&mut self, source: Source) -> Commands {
        // TODO(sysint64): handle unwraps.
        let client_module_state = self.module_states.get_mut(module::CLIENT_ID).unwrap();
        client_module_state.get_commands(source)
    }

    /// Clear all commands from the root module.
    pub fn clear_commands(&mut self, source: Source) -> Result<(), &'static str> {
        // TODO(sysint64): handle unwraps.
        let client_module_state = self.module_states.get_mut(module::CLIENT_ID).unwrap();
        client_module_state.clear_commands(source)
    }

    /// Process all commands for all modules from source.
    /// This method will clear all commands from source for module.
    pub fn process_commands(&mut self, source: Source) -> Result<(), &'static str> {
        assert!(self.modules.len() == self.module_states.len());

        for (i, module) in self.modules.iter_mut().enumerate() {
            let mut state = self.module_states.get_mut(i).unwrap();

            match source {
                Source::GAPI => module.render(&mut state),
            }

            state.clear_commands(source)?;
        }

        Ok(())
    }
}
