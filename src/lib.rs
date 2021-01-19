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

use std::{ffi::CStr, os::raw::c_char};

use log;

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
    Commands::empty()
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_consume_commands() -> Commands {
    Commands::empty()
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_gapi_flush() {}

#[no_mangle]
pub extern "C" fn tech_paws_vm_flush() {}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_trace(message: *const c_char) {
    let str = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::trace!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_error(message: *const c_char) {
    let str = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::error!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_warn(message: *const c_char) {
    let str = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::warn!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_debug(message: *const c_char) {
    let str = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::debug!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_info(message: *const c_char) {
    let str = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    log::info!("{}", str);
}
