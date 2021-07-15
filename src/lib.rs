#![warn(missing_docs)]

//! Virtual machine memory management.

pub mod commands;
pub mod commands_bus;
pub mod commands_reader;
pub mod data;
pub mod gapi;
pub mod module;
pub mod state;

use std::{ffi::CStr, os::raw::c_char};

use commands::Source;

use crate::module::Module;
use data::{BytesBuffer, MutBytesBuffer};
use state::VMState;

static mut STATE: Option<VMState> = None;

/// Initialize VM State.
///
/// # Safety
///
/// Should call once in the main thread.
pub unsafe fn init() {
    STATE = Some(VMState::new());
    // let client_module = module::ClientModule::new();
    // register_module(Box::new(client_module));
}

/// Initialize VM State.
#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_init() {
    init();
}

/// Register a new module
pub fn register_module(module: Box<dyn Module>) {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.register_module(module);
}

/// Process all commands from all modules.
#[no_mangle]
pub extern "C" fn tech_paws_vm_process_commands() {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.process_commands(Source::Processor).unwrap();
}

/// Process all render commands from all modules.
#[no_mangle]
pub extern "C" fn tech_paws_vm_process_render_commands() {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.process_commands(Source::GAPI).unwrap();
}

/// Clear current iteration state - commands memory, frame memory etc.
#[no_mangle]
pub extern "C" fn tech_paws_vm_flush() {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.flush().unwrap();
}

// TODO(sysint64): Create API to lock with mutex data
/// Get commands buffer data.
#[no_mangle]
pub extern "C" fn tech_paws_vm_get_commands_buffer() -> MutBytesBuffer {
    let state = unsafe { STATE.as_mut().unwrap() };
    state.get_commands_buffer(Source::GAPI)
}

/// Preparing command for sending.
#[no_mangle]
pub unsafe extern "C" fn tech_paws_begin_command(
    address: *const c_char,
    source: Source,
    id: u64,
) -> *mut vm_buffers::c_api::BytesWriter {
    let state = STATE.as_mut().unwrap();
    let address: &str = CStr::from_ptr(address).to_str().unwrap();
    state.client_command_bus.begin_command(address, source, id)
}

/// Finish command and send it to `address`
#[no_mangle]
pub unsafe extern "C" fn tech_paws_end_command(address: *const c_char, source: Source) {
    let state = STATE.as_mut().unwrap();
    let address: &str = CStr::from_ptr(address).to_str().unwrap();
    state.client_command_bus.end_command(address, source);
}

/// Get client module id.
#[no_mangle]
pub extern "C" fn tech_paws_vm_client_id() -> BytesBuffer {
    BytesBuffer {
        base: module::CLIENT_ID.as_ptr(),
        size: module::CLIENT_ID.len() as u64,
    }
}

/// Log level: trace
#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_trace(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::trace!("{}", str);
}

/// Log level: error
#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_error(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::error!("{}", str);
}

/// Log level: warning
#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_warn(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::warn!("{}", str);
}

/// Log level: debug
#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_debug(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::debug!("{}", str);
}

/// Log level: info
#[no_mangle]
pub unsafe extern "C" fn tech_paws_vm_log_info(message: *const c_char) {
    let str = CStr::from_ptr(message).to_str().unwrap();
    log::info!("{}", str);
}
