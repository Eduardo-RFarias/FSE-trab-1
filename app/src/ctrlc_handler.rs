use rust_socketio::client::Client;
use std::{
    io::{Stdout, Write},
    process,
    sync::{Arc, Mutex},
};
use termion::{clear, cursor, raw::RawTerminal};

pub fn set(client: Arc<Mutex<Client>>, stdout: Arc<Mutex<RawTerminal<Stdout>>>) {
    ctrlc::set_handler(move || {
        let mut status: i32 = -1;

        if let Ok(client) = client.lock() {
            if let Ok(_) = client.disconnect() {
                status = 0;
            }
        }

        if let Ok(mut stdout) = stdout.lock() {
            write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
            write!(stdout, "{}", clear::All).unwrap();
            write!(stdout, "{}", cursor::Show).unwrap();
            stdout.flush().unwrap();
        }

        process::exit(status);
    })
    .unwrap();
}
