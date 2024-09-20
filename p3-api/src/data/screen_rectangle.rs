use super::p3_ptr::P3Pointer;
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct ScreenRectanglePtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> ScreenRectanglePtr<P3> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn set_width(&self, width: u32, _api: &P3) -> Result<(), P3ApiError> {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x08) as _;
        unsafe {
            *ptr = width;
        }

        Ok(())
    }
}

impl<P3: P3AccessApi> P3Pointer for ScreenRectanglePtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}

#[derive(Clone, Debug)]
pub struct ScreenRectangleArrayPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> ScreenRectangleArrayPtr<P3> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_screen_rectangle(&self, index: u32, api: &P3) -> Result<ScreenRectanglePtr<P3>, P3ApiError> {
        //TODO SECURITY enforce size
        Ok(ScreenRectanglePtr::new(self.get(index * 4, api)?))
    }
}

impl<P3: P3AccessApi> P3Pointer for ScreenRectangleArrayPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
