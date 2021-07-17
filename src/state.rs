//! Virtual machine state.

use std::{collections::HashMap, time::Instant};
use vm_buffers::IntoVMBuffers;
use vm_memory::BufferAccessor;

use crate::{commands::{self, Source}, data::MutBytesBuffer, module::{self, ClientEvent, MouseButton, StepState}};
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
        println!("Registered");
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
    pub fn process_commands(&mut self, source: Source) -> Result<bool, &'static str> {
        assert!(self.modules.len() == self.module_states.len());
        let mut render_update = false;

        let client_info = {
            let mut client_state = self.module_states.get_mut(module::CLIENT_ID).unwrap();

            client_state.client_info.events.clear();

            let mut client_info = client_state.client_info.clone();

            if source == Source::Processor {
                client_state.get_commands_new(Source::Processor, |commands_reader| {
                    while let Some(command) = commands_reader.next() {
                        match command.id {
                            commands::COMMAND_TOUCH_START => {
                                client_info.events.push(ClientEvent::MouseDown {
                                    button: MouseButton::read_from_buffers(command.bytes_reader),
                                    x: command.bytes_reader.read_u32() as f32,
                                    y: command.bytes_reader.read_u32() as f32,
                                });
                            }
                            commands::COMMAND_TOUCH_END => {
                                client_info.events.push(ClientEvent::MouseUp {
                                    button: MouseButton::read_from_buffers(command.bytes_reader),
                                    x: command.bytes_reader.read_u32() as f32,
                                    y: command.bytes_reader.read_u32() as f32,
                                });
                            }
                            commands::COMMAND_TOUCH_MOVE => {
                                client_info.events.push(ClientEvent::MouseMove {
                                    x: command.bytes_reader.read_u32() as f32,
                                    y: command.bytes_reader.read_u32() as f32,
                                });
                            }
                            _ => (),
                        }
                    }
                });
            }

            client_state.client_info = client_info.clone();
            client_info.clone()
        };

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
                    let step_state = module.step(&mut state);
                    state.client_info = client_info.clone();
                    state.clear_commands(Source::Processor)?;
                    render_update = render_update || step_state == StepState::RenderUpdate;
                }
            }
        }

        Ok(render_update)
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
