extern crate num_derive;

pub mod data;
pub mod facility;
pub mod game_world;
pub mod town;
pub mod ui;

// https://stackoverflow.com/a/28175593/1569755
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().take_while(|c| **c != 0).map(|&c| c as char).collect()
}
