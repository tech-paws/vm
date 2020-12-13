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

use state::VMState;

use data::Commands;

static mut STATE: Option<VMState> = None;

/// In what allocator put your data
#[repr(C)]
pub enum Source {
    /// GAPI Allocator
    GAPI = 0,
}

/// Initialize VM State.
#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE = Some(VMState::new());
}

/// Get all commands from the source.
pub extern "C" fn get_commands(_source: Source) -> Commands {
    todo!()
}

/// Clear all commands from the source.
pub extern "C" fn clear_commands(_source: Source) {
    todo!()
}
