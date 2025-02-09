use std::{fs, ops::Not, path::Path};

use bitvec::{field::BitField, order::Lsb0, slice::BitSlice, view::BitView};
use log::{debug, info, trace};

const DECOMPRESS_TABLE: [DecompressTableEntry; 8] = [
    DecompressTableEntry { field_0: 1, field_4: 0x00 },
    DecompressTableEntry { field_0: 2, field_4: 0x02 },
    DecompressTableEntry { field_0: 3, field_4: 0x06 },
    DecompressTableEntry { field_0: 4, field_4: 0x0e },
    DecompressTableEntry { field_0: 5, field_4: 0x1e },
    DecompressTableEntry { field_0: 6, field_4: 0x3e },
    DecompressTableEntry { field_0: 7, field_4: 0x7e },
    DecompressTableEntry { field_0: 8, field_4: 0xfe },
];

const BITMASKS_TABLE_1: [u32; 8] = [0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff];
const BITMASKS_TABLE_2: [u32; 32] = [
    0x00,
    0x01,
    0x03,
    0x07,
    0x0f,
    0x1f,
    0x3f,
    0x7f,
    0xff,
    0x1ff,
    0x3ff,
    0x7ff,
    0xfff,
    0x1ff,
    0x3fff,
    0x7fff,
    0xffff,
    0x1_ffff,
    0x3_ffff,
    0x7_ffff,
    0xf_ffff,
    0x1f_ffff,
    0x3f_ffff,
    0x7f_ffff,
    0xff_ffff,
    0x1ff_ffff,
    0x3ff_ffff,
    0x7ff_ffff,
    0xfff_ffff,
    0x1fff_ffff,
    0x3FFFFFFF,
    0x7FFFFFFF,
];

struct DecompressTableEntry {
    field_0: u32,
    field_4: u32,
}

struct BitReader<'a> {
    data: &'a BitSlice<u8, Lsb0>,
    pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader {
            data: data.view_bits(),
            pos: 0,
        }
    }

    pub fn read<T: bitvec::macros::internal::funty::Integral>(&mut self, len: usize) -> T {
        let value = self.data[self.pos..self.pos + len].load::<T>();
        self.pos += len;
        value
    }

    pub fn read_bool(&mut self) -> bool {
        let result = self.data[self.pos..self.pos + 1].load::<u8>() == 1;
        self.pos += 1;
        result
    }

    pub fn read_u3(&mut self) -> u8 {
        let result = self.data[self.pos..self.pos + 3].load();
        self.pos += 3;
        result
    }

    pub fn read_u8(&mut self) -> u8 {
        let result = self.data[self.pos..self.pos + 8].load();
        self.pos += 8;
        result
    }

    pub fn read_i32(&mut self) -> i32 {
        let result = self.data[self.pos..self.pos + 32].load();
        self.pos += 32;
        result
    }
}

pub fn decompress_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let path = path.as_ref();
    info!("Reading {:?}", path);
    decompress(&fs::read(path).unwrap())
}

fn decompress(input: &[u8]) -> Vec<u8> {
    let mut reader = BitReader::new(input);
    let len = reader.read_i32();
    assert!(len > 0); //TODO implement negative length special case if required
    let mut output = Vec::with_capacity(len as usize);
    debug!("Parsing {len} bytes");
    while output.len() < len as usize {
        if reader.read_bool() {
            let compressed_chunk_type = reader.read_u3() as usize;
            let thingy_len = DECOMPRESS_TABLE[compressed_chunk_type].field_0;
            let thingy = reader.read::<u8>(thingy_len as usize) as u32;
            trace!("compressed_chunk_type={compressed_chunk_type:#04x} thingy_len={thingy_len} thingy={thingy}");

            let negated_value = DECOMPRESS_TABLE[compressed_chunk_type].field_4 + (BITMASKS_TABLE_1[compressed_chunk_type] & thingy);
            let offset = negated_value.not() as i32;
            trace!("negated={negated_value} offset={offset}");

            let mut elements = 1;
            let mut output_elements = 2;
            loop {
                elements += 1;
                let wat = reader.read::<u8>(elements) as u32;
                let why = BITMASKS_TABLE_1[elements] & wat;
                output_elements += why;
                if why != BITMASKS_TABLE_2[elements] {
                    trace!("Breaking loop: {why:#010x} != {:#010x}", BITMASKS_TABLE_2[elements]);
                    break;
                }
            }

            // Repeat slice
            let repeated_value_index = (output.len() as i32 + offset) as usize;
            for i in 0..output_elements {
                let value = output[repeated_value_index + i as usize];
                trace!("Pushing value {value:#04x} (decompressed)");
                output.push(value);
            }
        } else {
            let value = reader.read_u8();
            trace!("Pushing value {value:#04x} (regular)");
            output.push(value)
        }
    }
    output
}
