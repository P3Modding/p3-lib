use crate::{p3_access_api::P3AccessApi, P3ApiError};

pub trait P3Pointer {
    fn get_address(&self) -> u32;
    fn get<T: Sized, P3: P3AccessApi>(&self, offset: u32, api: &mut P3) -> Result<T, P3ApiError> {
        let address = self.get_address() + offset;
        get_from(address, api)
    }

}

pub fn get_from<T: Sized, P3: P3AccessApi>(address: u32, api: &mut P3) -> Result<T, P3ApiError> {
    let mut buf: Vec<u8> = vec![0; std::mem::size_of::<T>()];
    api.read_memory(address, &mut buf)?;
    let t: T = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
    Ok(t)
}
