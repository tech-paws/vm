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

use module::{BenchmarkModule, ClientModule};
use state::VMState;

static mut STATE: Option<VMState> = None;

/// Initialize VM State.
#[no_mangle]
pub unsafe extern "C" fn init() {
    let mut state = VMState::new();

    // TODO(sysint64): register modules for demo
    state.register_module(Box::new(ClientModule::new()));
    state.register_module(Box::new(BenchmarkModule::new()));

    STATE = Some(state);
}
