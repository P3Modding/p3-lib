use std::{ffi::c_void, mem};

use crate::data::p3_ptr::P3Pointer;

#[repr(u32)]
pub enum TextMode {
    AlignCenter = 0,
    AlignLeft = 1,
    AlignRight = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct DdrawFontPtr {
    pub address: u32,
}

impl DdrawFontPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn next_office_id(&self) -> u16 {
        unsafe { self.get(0x2ca) }
    }
}

impl P3Pointer for DdrawFontPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

#[derive(Debug)]
pub struct DdrawFontContainerPtr {
    pub address: u32,
}

impl DdrawFontContainerPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_font(&self) -> DdrawFontPtr {
        unsafe { DdrawFontPtr::new(self.get(0x98)) }
    }
}

impl P3Pointer for DdrawFontContainerPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

pub fn ddraw_set_text_mode(mode: TextMode) {
    let function: extern "cdecl" fn(mode: u32) = unsafe { mem::transmute(0x004BBA10) };
    function(mode as u32)
}

pub fn ddraw_set_font(font: DdrawFontPtr) {
    let function: extern "cdecl" fn(font: *const c_void) = unsafe { mem::transmute(0x004BB8F0) };
    function(font.address as _)
}

pub fn get_normal_font() -> DdrawFontPtr {
    DdrawFontContainerPtr::new(0x006DCDD0).get_font()
}

pub fn get_header_font() -> DdrawFontPtr {
    DdrawFontContainerPtr::new(0x006DCD28).get_font()
}

pub fn get_scrollmap_town_name_font() -> DdrawFontPtr {
    DdrawFontContainerPtr::new(0x006DCF20).get_font()
}
