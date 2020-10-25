extern "C" {
    pub fn virtual_alloc(size: i32) -> *const u8;
}

#[cfg(test)]
mod tests {
    use crate::virtual_alloc;

    #[test]
    fn it_works() {
        unsafe {
            assert!(!virtual_alloc(40).is_null());
        }
    }
}
