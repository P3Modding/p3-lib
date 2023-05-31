use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    num::ParseIntError,
    thread,
};

use log::{debug, info};

use crate::{Context, CONTEXT};

pub fn run_server() {
    info!("run_server()");
    let listener = TcpListener::bind("127.0.0.1:30123").unwrap();

    for stream in listener.incoming() {
        debug!("New control connection");
        thread::spawn(|| handle_client(stream.unwrap()));
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();

    debug!("Start handling client");
    while let Ok(bytes_read) = reader.read_line(&mut buf) {
        debug!("Received {} ({} bytes read)", buf.strip_suffix('\n').unwrap(), bytes_read);
        if bytes_read == 0 {
            info!("Client disconnected");
            break;
        }
        let decoded = decode_hex(&buf[0..40]).unwrap();
        let mut mg = CONTEXT.try_lock().unwrap();
        let context: &mut Context = mg.as_mut().unwrap();
        context.sender.send(decoded).unwrap();
        buf.clear()
    }
}

// Credits to https://stackoverflow.com/a/52992629/1569755
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16)).collect()
}
