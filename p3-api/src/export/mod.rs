#![allow(clippy::len_without_is_empty)]
use crate::{
    data::{enums::TownId, game_world::GameWorldPtr},
    p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi,
    P3ApiError,
};

#[repr(C)]
pub struct ByteBuffer {
    ptr: *mut u8,
    length: i32,
    capacity: i32,
}

#[repr(C)]
pub struct TownData {
    pub storage: StorageData,
    pub get_daily_consumptions_citizens: [i32; 0x18],
    pub offices: ByteBuffer,
}

#[repr(C)]
pub struct OfficeData {
    pub merchant_id: u16,
    pub storage: StorageData,
}

#[repr(C)]
pub struct StorageData {
    pub wares: [i32; 0x18],
    pub daily_consumption_businesses: [i32; 0x18],
    pub daily_production: [i32; 0x18],
}

impl TownData {
    pub fn read(raw_town_id: TownId, api: &OpenProcessP3AccessApi) -> Result<Option<Self>, P3ApiError> {
        let game_word = GameWorldPtr::new();
        let town = match game_word.get_town(raw_town_id, api)? {
            Some(town) => town,
            None => return Ok(None),
        };
        let mut offices = vec![];
        let mut office_index = town.get_first_office_id(api)?;
        while office_index < game_word.get_offices_count(api)? {
            let office = game_word.get_office(office_index, api)?;
            let storage = office.get_storage();
            offices.push(OfficeData {
                merchant_id: office.get_merchant_id(api)?,
                storage: StorageData {
                    wares: storage.get_wares(api)?,
                    daily_consumption_businesses: storage.get_daily_consumptions_businesses(api)?,
                    daily_production: storage.get_daily_production(api)?,
                },
            });
            office_index = office.next_office_id(api)?;
        }

        let town_storage = town.get_storage();
        Ok(Some(TownData {
            storage: StorageData {
                wares: town_storage.get_wares(api)?,
                daily_consumption_businesses: town_storage.get_daily_consumptions_businesses(api)?,
                daily_production: town_storage.get_daily_production(api)?,
            },
            get_daily_consumptions_citizens: town.get_daily_consumptions_citizens(api)?,
            offices: ByteBuffer::from_vec_struct::<OfficeData>(offices),
        }))
    }
}

impl ByteBuffer {
    pub fn len(&self) -> usize {
        self.length.try_into().expect("buffer length negative or overflowed")
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        let length = i32::try_from(bytes.len()).expect("buffer length cannot fit into a i32.");
        let capacity = i32::try_from(bytes.capacity()).expect("buffer capacity cannot fit into a i32.");

        // keep memory until call delete
        let mut v = std::mem::ManuallyDrop::new(bytes);

        Self {
            ptr: v.as_mut_ptr(),
            length,
            capacity,
        }
    }

    pub fn from_vec_struct<T: Sized>(bytes: Vec<T>) -> Self {
        let element_size = std::mem::size_of::<T>() as i32;

        let length = (bytes.len() as i32) * element_size;
        let capacity = (bytes.capacity() as i32) * element_size;

        let mut v = std::mem::ManuallyDrop::new(bytes);

        Self {
            ptr: v.as_mut_ptr() as *mut u8,
            length,
            capacity,
        }
    }

    pub fn destroy_into_vec(self) -> Vec<u8> {
        if self.ptr.is_null() {
            vec![]
        } else {
            let capacity: usize = self.capacity.try_into().expect("buffer capacity negative or overflowed");
            let length: usize = self.length.try_into().expect("buffer length negative or overflowed");

            unsafe { Vec::from_raw_parts(self.ptr, length, capacity) }
        }
    }

    pub fn destroy_into_vec_struct<T: Sized>(self) -> Vec<T> {
        if self.ptr.is_null() {
            vec![]
        } else {
            let element_size = std::mem::size_of::<T>() as i32;
            let length = (self.length * element_size) as usize;
            let capacity = (self.capacity * element_size) as usize;

            unsafe { Vec::from_raw_parts(self.ptr as *mut T, length, capacity) }
        }
    }

    pub fn destroy(self) {
        drop(self.destroy_into_vec());
    }
}
