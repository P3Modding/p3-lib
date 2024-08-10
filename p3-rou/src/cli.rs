use std::fs;

use log::LevelFilter;

pub fn main() {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Debug).env().init().unwrap();
    let data = p3_rou::read_rou("tests/a-unloadall.rou");
    let stops = data.len() / 220;
    for i in 0..stops {
        fs::write(format!("a-unloadall.{}.roustop", i), &data[i * 220..i * 220 + 220]).unwrap();
    }
}
