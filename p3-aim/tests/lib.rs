use std::{fs, io::Write};

use log::LevelFilter;

#[test]
fn test_1bpp() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
    let file = p3_aim::read_aim_file("./tests/X_holk_a.aim").unwrap();
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tests/X_holk_a.raw")
        .unwrap()
        .write_all(&file.data)
        .unwrap();
}

#[test]
fn test_4bpp() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
    let file = p3_aim::read_aim_file("./tests/page_schiffe.aim").unwrap();
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tests/page_schiffe.raw")
        .unwrap()
        .write_all(&file.data)
        .unwrap();
}

#[test]
fn test_wat() {
    // 1bpp
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
    let file = p3_aim::read_aim_file("./tests/Stadttor.aim").unwrap();
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tests/Stadttor.raw")
        .unwrap()
        .write_all(&file.data)
        .unwrap();
}

#[test]
fn test_wat2() {
    // 1bpp
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
    let file = p3_aim::read_aim_file("./tests/X_schniggeuntenno.aim").unwrap();
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tests/X_schniggeuntenno.raw")
        .unwrap()
        .write_all(&file.data)
        .unwrap();
}
