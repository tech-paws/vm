#![warn(missing_docs)]

//! Virtual machine memory management.

pub mod allocator;
pub mod c_api;
pub mod commands;
pub mod commands_bus;
pub mod data;
pub mod gapi;
pub mod module;
pub mod state;

use crate::module::Module;
use data::Commands;
use state::VMState;

static mut STATE: Option<VMState> = None;

/// Initialize VM State.
///
/// # Safety
///
/// Should call once in the main thread.
pub unsafe fn init() {
    STATE = Some(VMState::new());
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_init() {
    STATE = Some(VMState::new());
}

/// Register a new module
pub fn register_module(module: Box<dyn Module>) {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.register_module(module);
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_process_commands() {}

#[no_mangle]
pub extern "C" fn tech_paws_vm_process_render_commands() {}

#[no_mangle]
pub extern "C" fn tech_paws_vm_consume_gapi_commands() -> Commands {
    todo!()
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_consume_commands() -> Commands {
    todo!()
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_gapi_flush() {}

#[no_mangle]
pub extern "C" fn tech_paws_vm_flush() {}
