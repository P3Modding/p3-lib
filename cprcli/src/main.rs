/// Inspired by https://gist.github.com/javiercantero/e1042ca2cbb072599c98028c207689fe
use std::{
    backtrace::Backtrace,
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Seek, Write},
    path::{Path, PathBuf},
};

use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;
use log::{debug, error, LevelFilter};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Operation {
    Extract,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long, required(true))]
    input_file: PathBuf,

    /// Output path
    #[arg(short, long, required(false))]
    output_path: Option<PathBuf>,

    /// Operation
    #[clap(value_enum, default_value_t=Operation::Extract)]
    operation: Operation,
}

#[derive(Debug)]
pub enum CprCliError {
    IoError(io::Error),
}

#[derive(Clone, Debug)]
struct CprChunkHeader {
    index_size: u32,
    _unknown: u32,
    files: u32,
    data_size: u32,
}

#[derive(Clone, Debug)]
struct CprIndexEntry {
    offset: u32,
    size: u32,
    _unknown: u32,
    file_path: String,
}

fn main() {
    let args = Args::parse();
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();

    if !args.input_file.exists() {
        error!("Input file does not exist");
        return;
    }

    if !args.input_file.is_file() {
        error!("Input file is not a file");
        return;
    }

    let output_path = args.output_path.unwrap_or_else(|| {
        let mut path = args.input_file.clone();
        if path.extension().is_some() {
            path.set_extension(OsStr::new(""));
        } else {
            let input_file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
            path.pop();
            path.push(format!("{}_extract", input_file_name));
        }
        path
    });
    extract(args.input_file, output_path).unwrap();
}

fn extract(input_file: PathBuf, output_path: PathBuf) -> Result<(), CprCliError> {
    let f = File::open(input_file)?;
    let mut reader = BufReader::new(f);
    let mut chunk_pos = 0x20;
    reader.seek(io::SeekFrom::Start(chunk_pos))?;
    while !reader.fill_buf().unwrap().is_empty() {
        let chunk_header = read_cpr_chunk_header(&mut reader)?;
        debug!("{:08x?}", chunk_header);
        let mut index_entries = vec![];

        for _ in 0..chunk_header.files {
            let index_entry = read_cpr_index_entry(&mut reader)?;
            debug!("{:08x?}", index_entry);
            index_entries.push(index_entry);
        }

        for index_entry in &index_entries {
            reader.seek(io::SeekFrom::Start(index_entry.offset.try_into().unwrap()))?;
            let mut path = output_path.clone();
            path.push(Path::new(&index_entry.file_path));
            let prefix = path.parent().unwrap();
            fs::create_dir_all(prefix).unwrap();
            let mut file = File::create(&path)?;
            let mut buf = vec![0u8; index_entry.size.try_into().unwrap()];
            reader.read_exact(&mut buf)?;
            file.write_all(&buf)?;
        }

        chunk_pos += (chunk_header.index_size + chunk_header.data_size) as u64;
        reader.seek(io::SeekFrom::Start(chunk_pos))?;
    }

    Ok(())
}

fn read_cpr_chunk_header(reader: &mut BufReader<File>) -> Result<CprChunkHeader, CprCliError> {
    Ok(CprChunkHeader {
        index_size: reader.read_u32::<LittleEndian>()?,
        _unknown: reader.read_u32::<LittleEndian>()?,
        files: reader.read_u32::<LittleEndian>()?,
        data_size: reader.read_u32::<LittleEndian>()?,
    })
}

fn read_cpr_index_entry(reader: &mut BufReader<File>) -> Result<CprIndexEntry, CprCliError> {
    Ok(CprIndexEntry {
        offset: reader.read_u32::<LittleEndian>()?,
        size: reader.read_u32::<LittleEndian>()?,
        _unknown: reader.read_u32::<LittleEndian>()?,
        file_path: read_latin1_str(reader)?,
    })
}

fn read_latin1_str(reader: &mut BufReader<File>) -> Result<String, CprCliError> {
    let mut buffer = Vec::new();
    reader.read_until(0, &mut buffer)?;
    buffer.pop();
    Ok(latin1_to_string(&buffer))
}

// https://stackoverflow.com/a/28175593/1569755
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}

impl From<io::Error> for CprCliError {
    fn from(value: io::Error) -> Self {
        let bt = Backtrace::force_capture();
        error!("{}", bt);
        CprCliError::IoError(value)
    }
}
