use std::ffi::CString;
use std::sync::Mutex;

use log::trace;
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
    UnknownPixelWidth(u32, u32),
}

#[derive(Clone, Debug)]
pub enum PixelWidth {
    One,
    Four,
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
        let path_ptr = CString::new(path).unwrap();
        ffi::aim_convert_file(&mut raw_aim_file, path_ptr.as_ptr());
        trace!("Converted aim file: {:#x?}", raw_aim_file);

        if raw_aim_file.buf_ptr.is_null() {
            return Err(P3AimError::ParsingFailed);
        }

        let pixel_width = get_pixel_width(&raw_aim_file)?;
        let height = raw_aim_file.height;
        let width = match pixel_width {
            PixelWidth::One => raw_aim_file.width2,
            PixelWidth::Four => raw_aim_file.width1,
        };
        let pixels = width * height;
        let mut data = Vec::with_capacity(pixels.try_into().unwrap());

        if raw_aim_file.bytes_per_pixel1 == 1 && raw_aim_file.bytes_per_pixel2 == 1 {
            for i in 0..pixels {
                let pixel_id = *raw_aim_file.buf_ptr.offset(i.try_into().unwrap());
                let pixel = *raw_aim_file
                    .palette_ptr
                    .offset(pixel_id.try_into().unwrap());
                data.extend_from_slice(&pixel.to_le_bytes());
            }
        } else if raw_aim_file.bytes_per_pixel1 == 4 && raw_aim_file.bytes_per_pixel2 == 4 {
            let dword_ptr = raw_aim_file.buf_ptr as *const u32;
            for i in 0..pixels {
                let pixel = *dword_ptr.offset(i.try_into().unwrap());
                data.extend_from_slice(&pixel.to_le_bytes());
            }
        } else {
            return Err(P3AimError::UnknownPixelWidth(
                raw_aim_file.bytes_per_pixel1,
                raw_aim_file.bytes_per_pixel2,
            ));
        }

        let parsed_file = ParsedAimFile {
            data,
            width,
            height,
        };

        Ok(parsed_file)
    }
}

fn get_pixel_width(raw_aim_file: &RawAimFile) -> Result<PixelWidth, P3AimError> {
    if raw_aim_file.bytes_per_pixel1 == 1 && raw_aim_file.bytes_per_pixel2 == 1 {
        Ok(PixelWidth::One)
    } else if raw_aim_file.bytes_per_pixel1 == 4 && raw_aim_file.bytes_per_pixel2 == 4 {
        Ok(PixelWidth::Four)
    } else {
        Err(P3AimError::UnknownPixelWidth(
            raw_aim_file.bytes_per_pixel1,
            raw_aim_file.bytes_per_pixel2,
        ))
    }
}
