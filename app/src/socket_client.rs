use crate::constants::{CLIENT_HEADER, SERVER_ADDRESS};
use rust_socketio::{client::Client, ClientBuilder};
use std::{
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn create() -> Arc<Mutex<Client>> {
    let client_builder = ClientBuilder::new(SERVER_ADDRESS)
        .opening_header(CLIENT_HEADER.key, CLIENT_HEADER.value)
        .reconnect_on_disconnect(true)
        .max_reconnect_attempts(10);

    let mut connection: Option<Client> = None;

    for _ in 0..10 {
        match client_builder.clone().connect() {
            Ok(conn) => {
                connection = Some(conn);
                break;
            }
            Err(_) => {
                println!("Error connecting to the server. Retrying...");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    if connection.is_none() {
        println!("Failed to connect to the server. Exiting...");
        process::exit(1);
    }

    Arc::new(Mutex::new(connection.unwrap()))
}
