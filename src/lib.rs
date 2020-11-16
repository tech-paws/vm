#![warn(missing_docs)]

//! Virtual machine memory management.

pub mod allocator;
pub mod c_api;
pub mod data;
pub mod commands;
pub mod gapi;

// /// Render state memory allocator.
// pub mod render_state {
//     use crate::RegionAllocator;
//     use lazy_static::lazy_static;
//     use std::sync::Mutex;

//     lazy_static! {
//         static ref ALLOCATOR: Mutex<RegionAllocator> = Mutex::new(RegionAllocator::new(1024));
//     }

//     /// Allocate chunk of memory with particular size using global allocator.
//     /// returns the base address of the allocated chunk of memory.
//     ///
//     /// # Errors
//     ///
//     /// If the memory is run out, then this call will return an error.
//     ///
//     /// # Panics
//     ///
//     /// This function might panic when called if the allocator lock is already
//     /// held by the current thread.
//     pub unsafe fn alloc(size: usize) -> Result<*mut u8, &'static str> {
//         let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
//         allocator.alloc(size)
//     }

//     /// Free all memory.
//     ///
//     /// # Panics
//     ///
//     /// This function might panic when called if the allocator lock is already
//     /// held by the current thread.
//     pub unsafe fn clear() -> Result<(), &'static str> {
//         let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
//         allocator.clear()
//     }
// }

// /// Render commands memory allocator.
// pub mod render_commands {
//     use crate::RegionAllocator;
//     use lazy_static::lazy_static;
//     use std::sync::Mutex;

//     lazy_static! {
//         static ref ALLOCATOR: Mutex<RegionAllocator> = Mutex::new(RegionAllocator::new(1024));
//     }

//     /// Allocate chunk of memory with particular size, returns the base address of
//     /// the allocated chunk of memory.
//     ///
//     /// # Errors
//     ///
//     /// If the memory is run out, then this call will return an error.
//     ///
//     /// # Panics
//     ///
//     /// This function might panic when called if the allocator lock is already
//     /// held by the current thread.
//     pub unsafe fn alloc(size: usize) -> Result<*mut u8, &'static str> {
//         let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
//         allocator.alloc(size)
//     }

//     /// Free all memory.
//     ///
//     /// # Panics
//     ///
//     /// This function might panic when called if the allocator lock is already
//     /// held by the current thread.
//     pub unsafe fn clear() -> Result<(), &'static str> {
//         let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
//         allocator.clear()
//     }

//     // pub unsafe fn push_command(command: Command) -> Result<(), &'static str> {
//     // Ok(())
//     // }
// }
