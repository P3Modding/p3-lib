use super::{p3_ptr::P3Pointer, screen_game_ini_anim::ScreenGameIniAnimPtr};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

pub const CLASS27_PTR_ADDRESS: u32 = 0x006CBB40;

#[derive(Clone, Debug)]
pub struct Class27Ptr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> Class27Ptr<P3> {
    pub fn new(api: &P3) -> Result<Self, P3ApiError> {
        Ok(Self {
            address: api.read_u32(CLASS27_PTR_ADDRESS)?,
            api_type: PhantomData,
        })
    }

    pub fn get_anim_42(&self) -> ScreenGameIniAnimPtr<P3> {
        ScreenGameIniAnimPtr::new(self.address + 0xa90)
    }

    pub fn get_anim_44(&self) -> ScreenGameIniAnimPtr<P3> {
        ScreenGameIniAnimPtr::new(self.address + 0xdf0)
    }
}

impl<P3: P3AccessApi> P3Pointer for Class27Ptr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
