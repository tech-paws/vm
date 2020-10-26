#[repr(C)]
pub struct RegionMemoryBuffer {
    pub size: u64,
    pub base: *const u8,
    pub offset: usize,
}

extern "C" {
    pub fn virtual_alloc(size: i32) -> *mut u8;

    pub fn create_region_memory_buffer(size: u64) -> RegionMemoryBuffer;

    pub fn region_memory_buffer_emplace_region(buffer: *mut RegionMemoryBuffer, size: u64) -> RegionMemoryBuffer;

    pub fn region_memory_buffer_alloc(buffer: *mut RegionMemoryBuffer, size: u64) -> *mut u8;

    pub fn region_memory_buffer_free(buffer: *mut RegionMemoryBuffer);
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
            let sub_region = region_memory_buffer_emplace_region(&mut region as *mut RegionMemoryBuffer, 512);

            assert!(!sub_region.base.is_null());
            assert_eq!(512, region.offset);
        }
    }
}
