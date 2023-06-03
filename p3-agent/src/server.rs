use std::{
    io::{self, BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    num::ParseIntError,
    sync::atomic::Ordering,
    thread::{self, sleep},
    time::Duration,
};

use log::{debug, error, info};

use crate::{AgentContext, CONTEXT, STATUS, STATUS_SHUTDOWN, STATUS_SHUTDOWN_FINISHED};

#[derive(Debug)]
pub enum ClientConnectionError {
    IOError(io::Error),
    ClientDisconnected,
    InvalidOperation,
}

#[derive(Debug)]
pub enum ServerError {
    IOError(io::Error),
}

pub fn run_server() {
    info!("run_server()");

    if let Err(e) = accept_connections() {
        error!("accept_connections failed: {:?}", e);
    }

    info!("run_server() stopped");
    STATUS.store(STATUS_SHUTDOWN_FINISHED, Ordering::SeqCst);
}

fn accept_connections() -> Result<(), ServerError> {
    let mut threads = vec![];
    let listener = TcpListener::bind("127.0.0.1:30123")?;
    listener.set_nonblocking(true)?;

    loop {
        if STATUS.load(Ordering::SeqCst) == STATUS_SHUTDOWN {
            break;
        }

        let (stream, address) = match listener.accept() {
            Ok((stream, address)) => (stream, address),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => return Err(ServerError::IOError(e)),
        };

        threads.push(thread::spawn(move || match handle_client(stream, address) {
            Ok(_) => debug!("handle_client finished"),
            Err(e) => error!("handle_client failed: {:?}", e),
        }));
    }

    debug!("Joining {} threads", threads.len());
    for thread in threads {
        debug!("Joining thread {:?} (is_finished={})", thread, thread.is_finished());
        let _ = thread.join();
        debug!("Joining thread done");
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, address: SocketAddr) -> Result<(), ClientConnectionError> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();

    debug!("handle_client {:?}", address);
    loop {
        if STATUS.load(Ordering::SeqCst) == STATUS_SHUTDOWN {
            break;
        }

        let bytes_read = match reader.read_line(&mut buf) {
            Ok(bytes_read) => bytes_read,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(1));
                continue;
            }
            Err(e) => return Err(ClientConnectionError::IOError(e)),
        };
        debug!("Received {} ({} bytes read)", buf.strip_suffix('\n').unwrap(), bytes_read);

        if bytes_read == 0 {
            info!("Client disconnected");
            return Err(ClientConnectionError::ClientDisconnected);
        }

        if bytes_read != 41 {
            info!("Client sent invalid operation");
            return Err(ClientConnectionError::InvalidOperation);
        }

        let decoded = decode_hex(&buf[0..40]).map_err(|_| ClientConnectionError::InvalidOperation)?;
        let mut mg = CONTEXT.lock().unwrap();
        let context: &mut AgentContext = mg.as_mut().unwrap();
        context.sender.send(decoded).unwrap();
        stream.write_all("OK\n".as_bytes())?;
        buf.clear()
    }

    Ok(())
}

// Credits to https://stackoverflow.com/a/52992629/1569755
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16)).collect()
}

impl From<io::Error> for ClientConnectionError {
    fn from(value: io::Error) -> Self {
        ClientConnectionError::IOError(value)
    }
}

impl From<io::Error> for ServerError {
    fn from(value: io::Error) -> Self {
        ServerError::IOError(value)
    }
}
