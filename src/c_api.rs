//! C API to manage memory.

/// Continuous chunk of memory.
#[repr(C)]
pub struct RegionMemoryBuffer {
    /// Size of the chunk.
    pub size: u64,
    /// Base address for the chunk.
    pub base: *mut u8,
    /// Offset from the base address. Shows how many bytes are allocated.
    pub offset: usize,
}

extern "C" {
    /// Allocate virtual memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vm_memory_manager::c_api::*;
    /// assert!(!unsafe { virtual_alloc(40) }.is_null());
    /// ```
    pub fn virtual_alloc(size: i32) -> *mut u8;

    /// Allocate a contiguous chunk of memory of a specific size.
    ///
    /// If out of memory, than `base` address will be `null`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vm_memory_manager::c_api::*;
    /// let region = unsafe { create_region_memory_buffer(1024) };
    /// assert!(!region.base.is_null());
    /// ```
    pub fn create_region_memory_buffer(size: u64) -> RegionMemoryBuffer;

    /// Allocate a contiguous chunk of memory of a specific size inside another
    /// [`RegionMemoryBuffer`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vm_memory_manager::c_api::*;
    /// let mut region = unsafe { create_region_memory_buffer(1024) };
    /// let sub_region =
    ///     unsafe { region_memory_buffer_emplace_region(&mut region as *mut RegionMemoryBuffer, 512) };
    ///
    /// assert!(!sub_region.base.is_null());
    /// assert_eq!(512, region.offset);
    /// ```
    pub fn region_memory_buffer_emplace_region(
        buffer: *mut RegionMemoryBuffer,
        size: u64,
    ) -> RegionMemoryBuffer;

    /// Allocate chunck of memory inside the `buffer`.
    ///
    /// The function will return the base address on success and `null` if an error occurs,
    /// e.g. Out Of Memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vm_memory_manager::c_api::*;
    /// let mut region = unsafe { create_region_memory_buffer(1024) };
    /// let base = unsafe { region_memory_buffer_alloc(&mut region as *mut RegionMemoryBuffer, 100) };
    ///
    /// assert!(!base.is_null());
    /// assert_eq!(100, region.offset);
    /// ```
    pub fn region_memory_buffer_alloc(buffer: *mut RegionMemoryBuffer, size: u64) -> *mut u8;

    /// Emplace `data` to the `buffer`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vm_memory_manager::c_api::*;
    /// # use std::mem;
    /// let mut region = unsafe { create_region_memory_buffer(1024) };
    /// let value: i32 = 100;
    /// let value_ptr = &value as *const i32;
    /// let data = unsafe {
    ///     region_memory_buffer_emplace(
    ///         &mut region as *mut RegionMemoryBuffer,
    ///         mem::size_of::<i32>() as u64,
    ///         value_ptr as *const u8,
    ///     )
    /// };
    /// let value = unsafe { data.as_ref().unwrap() };
    /// assert_eq!(100, *value);
    /// ```
    pub fn region_memory_buffer_emplace(
        buffer: *mut RegionMemoryBuffer,
        size: u64,
        data: *const u8,
    ) -> *mut u8;

    /// Free `buffer` by move pointer to the `base` addres.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vm_memory_manager::c_api::*;
    /// let mut region = unsafe { create_region_memory_buffer(1024) };
    /// unsafe { region_memory_buffer_alloc(&mut region as *mut RegionMemoryBuffer, 100) };
    /// unsafe { region_memory_buffer_free(&mut region as *mut RegionMemoryBuffer) };
    /// assert_eq!(0, region.offset);
    /// ```
    pub fn region_memory_buffer_free(buffer: *mut RegionMemoryBuffer);
}
