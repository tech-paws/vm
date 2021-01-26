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

use std::{ffi::CStr, mem, os::raw::c_char};

use commands::Source;
use log;

use crate::module::Module;
use data::{Command, Commands};
use state::VMState;

static mut STATE: Option<VMState> = None;

/// Initialize VM State.
///
/// # Safety
///
/// Should call once in the main thread.
pub unsafe fn init() {
    STATE = Some(VMState::new());
    let client_module = module::ClientModule::new();
    register_module(Box::new(client_module));
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_init() {
    init();
}

/// Register a new module
pub fn register_module(module: Box<dyn Module>) {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.register_module(module);
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_process_commands() {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.process_commands(Source::Processor);
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_process_render_commands() {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.process_commands(Source::GAPI);
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_get_gapi_commands() -> Commands {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.get_commands(Source::GAPI)
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_get_commands() -> Commands {
    Commands::empty()
}

#[no_mangle]
pub extern "C" fn tech_paws_vm_gapi_flush() {}

#[no_mangle]
pub extern "C" fn tech_paws_vm_flush() {}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_trace(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::trace!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_error(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::error!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_warn(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::warn!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_debug(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::debug!("{}", str);
}

#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_info(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::info!("{}", str);
}
