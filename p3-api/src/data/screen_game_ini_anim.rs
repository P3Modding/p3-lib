use super::{
    p3_ptr::P3Pointer,
    screen_rectangle::{ScreenRectangleArrayPtr, ScreenRectanglePtr},
};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct ScreenGameIniAnimPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> ScreenGameIniAnimPtr<P3> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn set_pos_x(&self, x: u32, _api: &P3) -> Result<(), P3ApiError> {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x20) as _;
        unsafe {
            *ptr = x;
        }
        Ok(())
    }

    pub fn get_screen_rectangle_array_ptr(&self, api: &P3) -> Result<ScreenRectangleArrayPtr<P3>, P3ApiError> {
        self.get(0xa0, api)
    }

    pub fn get_screen_rectangle(&self, index: u32, api: &P3) -> Result<ScreenRectanglePtr<P3>, P3ApiError> {
        self.get_screen_rectangle_array_ptr(api)?.get_screen_rectangle(index, api)
    }
}

impl<P3: P3AccessApi> P3Pointer for ScreenGameIniAnimPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
