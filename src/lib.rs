#[repr(C)]
pub struct RegionMemoryBuffer {
    pub size: u64,
    pub base: *mut u8,
    pub offset: usize,
}

extern "C" {
    pub fn virtual_alloc(size: i32) -> *mut u8;

    pub fn create_region_memory_buffer(size: u64) -> RegionMemoryBuffer;

    pub fn region_memory_buffer_emplace_region(
        buffer: *mut RegionMemoryBuffer,
        size: u64,
    ) -> RegionMemoryBuffer;

    pub fn region_memory_buffer_alloc(buffer: *mut RegionMemoryBuffer, size: u64) -> *mut u8;

    pub fn region_memory_buffer_free(buffer: *mut RegionMemoryBuffer);
}

struct RegionAllocator {
    region: RegionMemoryBuffer,
}

unsafe impl Send for RegionAllocator {}

impl RegionAllocator {
    fn new(size: usize) -> Self {
        Self {
            region: unsafe { create_region_memory_buffer(size as u64) },
        }
    }

    unsafe fn alloc(&mut self, size: usize) -> Result<*mut u8, &'static str> {
        let data =
            region_memory_buffer_alloc(&mut self.region as *mut RegionMemoryBuffer, size as u64);

        if data.is_null() {
            Err("Out of memory")
        }
        else {
            Ok(data)
        }
    }

    unsafe fn clear(&mut self) -> Result<(), &'static str> {
        region_memory_buffer_free(&mut self.region as *mut RegionMemoryBuffer);
        Ok(())
    }
}

pub mod render_state {
    use crate::RegionAllocator;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref ALLOCATOR: Mutex<RegionAllocator> = Mutex::new(RegionAllocator::new(1024));
    }

    pub unsafe fn alloc(size: usize) -> Result<*mut u8, &'static str> {
        let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
        allocator.alloc(size)
    }

    pub unsafe fn clear() -> Result<(), &'static str> {
        let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
        allocator.clear()
    }
}

pub mod render_commands {
    use crate::RegionAllocator;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref ALLOCATOR: Mutex<RegionAllocator> = Mutex::new(RegionAllocator::new(1024));
    }

    pub unsafe fn alloc(size: usize) -> Result<*mut u8, &'static str> {
        let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
        allocator.alloc(size)
    }

    pub unsafe fn clear() -> Result<(), &'static str> {
        let mut allocator = ALLOCATOR.lock().expect("failed to get allocator");
        allocator.clear()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_virtual_alloc() {
        unsafe {
            assert!(!virtual_alloc(40).is_null());
        }
    }

    #[test]
    fn test_create_region_memory_buffer() {
        unsafe {
            let region = create_region_memory_buffer(1024);
            assert!(!region.base.is_null());
        }
    }

    #[test]
    fn test_alloc_in_region_buffer() {
        unsafe {
            let mut region = create_region_memory_buffer(1024);
            let base = region_memory_buffer_alloc(&mut region as *mut RegionMemoryBuffer, 100);

            assert!(!base.is_null());
            assert_eq!(100, region.offset);

            region_memory_buffer_free(&mut region as *mut RegionMemoryBuffer);

            assert_eq!(0, region.offset);
        }
    }

    #[test]
    fn test_region_memory_buffer_emplace_region() {
        unsafe {
            let mut region = create_region_memory_buffer(1024);
            let sub_region =
                region_memory_buffer_emplace_region(&mut region as *mut RegionMemoryBuffer, 512);

            assert!(!sub_region.base.is_null());
            assert_eq!(512, region.offset);
        }
    }
}
