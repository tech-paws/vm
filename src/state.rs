//! Virtual machine state.

use std::{collections::HashMap, time::Instant};
use vm_memory::BufferAccessor;

use crate::{commands::Source, data::MutBytesBuffer, module};
use crate::{
    commands_bus::CommandsBus,
    module::{Module, ModuleState},
};

/// State structure.
pub struct VMState {
    pub client_command_bus: CommandsBus,

    /// Connected modules.
    pub modules: Vec<Box<dyn Module>>,

    /// Module states.
    pub module_states: HashMap<&'static str, ModuleState>,
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
            client_command_bus: CommandsBus::new(),
            modules: Vec::new(),
            module_states: HashMap::new(),
        }
    }

    /// Register module in the virtual machine.
    pub fn register_module(&mut self, mut module: Box<dyn Module>) {
        assert!(self.modules.len() == self.module_states.len());

        let mut module_state = ModuleState::new(module.id());
        module.init(&mut module_state);

        self.module_states.insert(module.id(), module_state);
        self.modules.push(module);
    }

    ///
    pub fn step() {}

    ///
    pub fn render() {}

    /// Get commands from the root module.
    pub fn get_commands_buffer(&mut self, source: Source) -> MutBytesBuffer {
        // TODO(sysint64): handle unwraps.
        let client_module_state = self.module_states.get_mut(&module::CLIENT_ID).unwrap();
        let commands_allocator = match source {
            Source::GAPI => client_module_state.gapi_commands.allocator.lock(),
            Source::Processor => client_module_state.processor_commands.allocator.lock(),
        };

        // println!("Dump: -------------------------------------------------------------------------");

        // let bytes = unsafe {
        //     std::slice::from_raw_parts(
        //         commands_allocator.get_buffer_ptr(),
        //         commands_allocator.get_buffer_size() as usize,
        //     )
        // };
        // hexdump::hexdump(bytes);

        MutBytesBuffer {
            base: commands_allocator.get_buffer_ptr(),
            size: commands_allocator.get_buffer_size(),
        }
    }

    /// Process all commands for all modules from source.
    /// This method will clear all commands from source for module.
    pub fn process_commands(&mut self, source: Source) -> Result<(), &'static str> {
        assert!(self.modules.len() == self.module_states.len());

        for module in self.modules.iter_mut() {
            let mut state = self.module_states.get_mut(&module.id()).unwrap();

            match source {
                Source::GAPI => {
                    if !state.last_time_initialized {
                        state.last_time = Instant::now();
                        state.last_time_initialized = true;
                    }

                    state.delta_time = state.last_time.elapsed().as_secs_f32();
                    module.render(&mut state);
                    state.last_time = Instant::now();
                    state.clear_commands(Source::GAPI)?;
                }
                Source::Processor => {
                    state.process_scheduler();
                    module.step(&mut state);
                    state.clear_commands(Source::Processor)?;
                }
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), &'static str> {
        assert!(self.modules.len() == self.module_states.len());

        for module in self.modules.iter() {
            let state = self.module_states.get_mut(&module.id()).unwrap();
            state.clear_text_boundaries()?;
            state.clear_commands(Source::GAPI)?;
            state.clear_commands(Source::Processor)?;
        }

        Ok(())
    }
}
