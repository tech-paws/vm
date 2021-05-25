//! Virtual machine primitive datas.

use std::{mem, ptr::null};

/// Virtual machine command payload.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct BytesBuffer {
    /// Size of the data.
    pub size: u64,
    /// Base address of the data.
    pub base: *const u8,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MutBytesBuffer {
    /// Size of the data.
    pub size: u64,
    /// Base address of the data.
    pub base: *mut u8,
}

impl BytesBuffer {
    /// Create a new payload with a given array.
    pub fn new<T>(values: &[T]) -> Self {
        let base = values
            .first()
            .map(|value| value as *const T)
            .unwrap_or(null::<T>());

        BytesBuffer {
            size: (mem::size_of::<T>() as u64) * (values.len() as u64),
            base: base as *const u8,
        }
    }

    pub fn from_str(text: &str) -> Self {
        BytesBuffer {
            size: text.len() as u64,
            base: text.as_ptr(),
        }
    }

    pub fn from_string(text: &String) -> Self {
        BytesBuffer {
            size: text.len() as u64,
            base: text.as_ptr(),
        }
    }

    /// Create empty payload without any data.
    pub const EMPTY: BytesBuffer = BytesBuffer {
        size: 0,
        base: null::<u8>(),
    };
}
