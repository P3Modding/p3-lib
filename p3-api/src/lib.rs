#![allow(clippy::missing_safety_doc)]
extern crate num_derive;

pub mod class35;
pub mod data;
pub mod facility;
pub mod game_world;
pub mod merchant;
pub mod missions;
pub mod operation;
pub mod operations;
pub mod scheduled_tasks;
pub mod ship;
pub mod ships;
pub mod town;
pub mod ui;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// https://stackoverflow.com/a/28175593/1569755
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().take_while(|c| **c != 0).map(|&c| c as char).collect()
}

pub unsafe fn latin1_ptr_to_string(mut s: *const u8) -> String {
    let mut result = String::new();
    while *s != 0 {
        result.push(*s as char);
        s = s.add(1);
    }
    result
}
