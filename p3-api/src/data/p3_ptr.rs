use std::ptr;

pub trait P3Pointer {
    fn get_address(&self) -> u32;

    /// # Safety
    /// Only safe, if T indeed be read.
    unsafe fn get<T: Sized>(&self, offset: u32) -> T {
        let address = self.get_address() + offset;
        let mut buf: Vec<u8> = vec![0; std::mem::size_of::<T>()];
        ptr::copy(address as *const u8, buf.as_mut_ptr(), std::mem::size_of::<T>());
        let t: T = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
        t
    }

    /// # Safety
    /// Only safe, if T indeed be written.
    unsafe fn set<T: Sized>(&self, offset: u32, data: &T) {
        let address = self.get_address() + offset;
        ptr::copy(data as *const _ as _, address as *mut u8, std::mem::size_of::<T>());
    }
}
