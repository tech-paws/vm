use std::{mem, ops};

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

    pub fn region_memory_buffer_emplace(
        buffer: *mut RegionMemoryBuffer,
        size: u64,
        data: *const u8,
    ) -> *mut u8;

    pub fn region_memory_buffer_free(buffer: *mut RegionMemoryBuffer);
}

#[repr(C)]
pub struct CommandPayload {
    pub size: u64,
    pub base: *mut u8,
}

#[repr(C)]
pub struct Command {
    pub id: u64,
    pub payload: CommandPayload,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub const ZERO: Vec2f = Vec2f::new(0., 0.);

    pub const fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }
}

impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
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

    unsafe fn emplace_struct<T>(&mut self, value: T) -> Result<*mut T, &'static str> {
        let value_ptr = &value as *const T;
        let data = region_memory_buffer_emplace(
            &mut self.region as *mut RegionMemoryBuffer,
            mem::size_of::<T>() as u64,
            value_ptr as *const u8,
        );

        if data.is_null() {
            Err("Out of memory")
        }
        else {
            Ok(data as *mut T)
        }
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
    use crate::{Command, RegionAllocator};
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

    pub unsafe fn push_command(command: Command) -> Result<(), &'static str> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use assert_approx_eq::assert_approx_eq;

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

    #[test]
    fn test_region_allocator_emplace_struct() {
        unsafe {
            let mut allocator = RegionAllocator::new(1024);
            let vec = Vec2f::new(10., 20.);
            let vec = allocator.emplace_struct(vec).unwrap().as_ref().unwrap();

            assert_eq!(mem::size_of::<Vec2f>(), allocator.region.offset);
            assert_approx_eq!(10., vec.x);
            assert_approx_eq!(20., vec.y);
        }
    }
}
