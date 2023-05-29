use std::path::PathBuf;

use clap::Parser;
use glob::glob;
use image::{ImageBuffer, ImageError, RgbaImage};
use log::{debug, error, warn, LevelFilter};
use p3_aim::P3AimError;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Operation {
    AimToBmp,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input pattern, such as 'C:\Program Files (x86)\Patrician III\**\*.aim'
    #[arg(short, long, required(true))]
    input_pattern: String,

    /// Conversion operation
    #[clap(value_enum, default_value_t=Operation::AimToBmp)]
    operation: Operation,
}

#[derive(Debug)]
pub enum AimCliError {
    AimParserError(P3AimError),
    ImageError(ImageError),
}

pub fn main() {
    let args = Args::parse();
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();

    for entry in glob(&args.input_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Err(e) = aim_to_bmp(&path) {
                    error!("Failed to handle {:?}: {:?}", path, e)
                }
            }
            Err(e) => error!("{:?}", e),
        }
    }
}

fn aim_to_bmp(input_file: &PathBuf) -> Result<(), AimCliError> {
    debug!("Converting {:?}", input_file);
    let output_file = if let Some(extension) = input_file.extension() {
        if extension != "aim" {
            warn!("Unexpected file extension {:?}", extension);
        }

        let mut output_file = input_file.clone();
        output_file.set_extension("bmp");
        output_file
    } else {
        let mut output_file = input_file.clone();
        output_file.push("bmp");
        output_file
    };

    let converted_file = p3_aim::read_aim_file(input_file.as_os_str().to_str().unwrap())?;
    let mut bmp: RgbaImage = ImageBuffer::new(converted_file.width, converted_file.height);
    let mut i = 0;
    for y in 0..converted_file.height {
        for x in 0..converted_file.width {
            let pixel = image::Rgba([
                converted_file.data[i + 2],
                converted_file.data[i + 1],
                converted_file.data[i],
                converted_file.data[i + 3],
            ]);
            i += 4;
            bmp.put_pixel(x, y, pixel)
        }
    }

    bmp.save(output_file)?;

    Ok(())
}

impl From<P3AimError> for AimCliError {
    fn from(value: P3AimError) -> Self {
        AimCliError::AimParserError(value)
    }
}

impl From<ImageError> for AimCliError {
    fn from(value: ImageError) -> Self {
        AimCliError::ImageError(value)
    }
}
