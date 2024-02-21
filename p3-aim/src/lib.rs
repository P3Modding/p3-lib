use std::sync::Mutex;

use log::{debug, trace};
use p3_aim_sys::ffi;
use p3_aim_sys::raw_aim_file::RawAimFile;

static LOCK: Mutex<u32> = Mutex::new(0);

#[derive(Clone, Debug)]
pub struct ParsedAimFile {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub enum P3AimError {
    ParsingFailed,
    UnknownPixelEncoding(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PixelEncoding {
    Compact,
    Palette,
    NoAlpha,
    Simple,
}

fn str_to_latin1(s: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = s.chars().map(|c| c as u8).collect();
    vec.push(0);
    vec
}

/// Reads an AIM file into memory.
///
/// # Arguments
/// * `path` - Path to the input file
pub fn read_aim_file(path: &str) -> Result<ParsedAimFile, P3AimError> {
    unsafe {
        // AIM.dll is not thread-safe in the slightest
        let _lock = &mut *LOCK.lock().unwrap();

        let mut raw_aim_file = RawAimFile::default();
        ffi::aim_init(&mut raw_aim_file);
        ffi::aim_convert_file(&mut raw_aim_file, str_to_latin1(path).as_ptr() as _);
        trace!("Converted aim file: {:#x?}", raw_aim_file);

        if raw_aim_file.buf_ptr.is_null() {
            return Err(P3AimError::ParsingFailed);
        }

        let encoding = get_pixel_encoding(&raw_aim_file)?;
        debug!("Using encoding: {:?} ({}/{})", encoding, raw_aim_file.width1, raw_aim_file.width2);
        let mut data = vec![];

        match encoding {
            PixelEncoding::Compact => {
                // TODO this is probably wrong
                assert!(raw_aim_file.palette_ptr.is_null());
                let input_pixels = raw_aim_file.width1 * (raw_aim_file.height + 1);
                for i in 0..input_pixels {
                    let pixel = *raw_aim_file.buf_ptr.offset(i.try_into().unwrap());
                    let a = (pixel & 0b11) << 6;
                    let r = ((pixel >> 3) & 0b11) << 4;
                    let g = ((pixel >> 5) & 0b11) << 2;
                    let b = (pixel >> 5) & 0b11;
                    data.push(b);
                    data.push(g);
                    data.push(r);
                    data.push(a);
                }
                Ok(ParsedAimFile {
                    data,
                    width: raw_aim_file.width1,
                    height: raw_aim_file.height,
                })
            }
            PixelEncoding::Palette => {
                let mut i = 0;
                for _ in 0..raw_aim_file.height {
                    for _ in 0..raw_aim_file.width1 {
                        let pixel_id = *raw_aim_file.buf_ptr.offset(i.try_into().unwrap());
                        let pixel = *raw_aim_file.palette_ptr.offset(pixel_id.try_into().unwrap());
                        data.extend_from_slice(&pixel.to_le_bytes());
                        i += 1;
                    }
                    // Skip the not set bytes and replace with transparency
                    for _ in raw_aim_file.width1..raw_aim_file.width2 {
                        data.push(0xff);
                        data.push(0x00);
                        data.push(0xff);
                        data.push(0xff);
                        i += 1;
                    }
                }
                Ok(ParsedAimFile {
                    data,
                    width: raw_aim_file.width2,
                    height: raw_aim_file.height,
                })
            }
            PixelEncoding::NoAlpha => {
                assert!(raw_aim_file.palette_ptr.is_null());
                assert_eq!(raw_aim_file.width1 * 3, raw_aim_file.width2);
                let input_pixels = raw_aim_file.width1 * raw_aim_file.height;
                let mut i = 0;
                while i < input_pixels * 3 {
                    let r = *raw_aim_file.buf_ptr.offset(i.try_into().unwrap());
                    let g = *raw_aim_file.buf_ptr.offset((i + 1).try_into().unwrap());
                    let b = *raw_aim_file.buf_ptr.offset((i + 2).try_into().unwrap());
                    i += 3;
                    let a = 0xff;
                    data.push(b);
                    data.push(g);
                    data.push(r);
                    data.push(a);
                }
                Ok(ParsedAimFile {
                    data,
                    width: raw_aim_file.width1,
                    height: raw_aim_file.height,
                })
            }
            PixelEncoding::Simple => {
                assert!(raw_aim_file.palette_ptr.is_null());
                assert_eq!(raw_aim_file.width1 * 4, raw_aim_file.width2);
                let input_pixels = raw_aim_file.width1 * raw_aim_file.height;
                let dword_ptr = raw_aim_file.buf_ptr as *const u32;
                for i in 0..input_pixels {
                    let pixel = *dword_ptr.offset(i.try_into().unwrap());
                    data.extend_from_slice(&pixel.to_le_bytes());
                }
                Ok(ParsedAimFile {
                    data,
                    width: raw_aim_file.width1,
                    height: raw_aim_file.height,
                })
            }
        }
    }
}

fn get_pixel_encoding(raw_aim_file: &RawAimFile) -> Result<PixelEncoding, P3AimError> {
    match raw_aim_file.pixel_encoding {
        0 => Ok(PixelEncoding::Compact),
        1 => Ok(PixelEncoding::Palette),
        3 => Ok(PixelEncoding::NoAlpha),
        4 => Ok(PixelEncoding::Simple),
        _ => Err(P3AimError::UnknownPixelEncoding(raw_aim_file.pixel_encoding)),
    }
}
