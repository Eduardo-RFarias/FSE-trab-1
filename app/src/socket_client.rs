use crate::{
    constants::{CLIENT_HEADER, PARKING_LOT_STATE_EVENT, SERVER_ADDRESS},
    menus,
    models::ParkingLotDataPayload,
};
use rust_socketio::{client::Client, ClientBuilder, Payload};
use std::{
    io::Stdout,
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use termion::raw::RawTerminal;

pub fn create(
    stdout: Arc<Mutex<RawTerminal<Stdout>>>,
    parking_lot: Arc<Mutex<ParkingLotDataPayload>>,
) -> Arc<Mutex<Client>> {
    let mut client_builder = ClientBuilder::new(SERVER_ADDRESS)
        .opening_header(CLIENT_HEADER.key, CLIENT_HEADER.value)
        .reconnect_on_disconnect(true)
        .max_reconnect_attempts(10);

    client_builder = client_builder.on(PARKING_LOT_STATE_EVENT, move |payload, _| {
        let mut parking_lot = parking_lot.lock().unwrap();

        if let Payload::Text(data) = payload {
            *parking_lot = serde_json::from_str(&data[0].to_string()).unwrap();
        } else {
            panic!("Payload is not text");
        }

        let mut stdout = stdout.lock().unwrap();
        menus::dashboard(&mut stdout, &parking_lot);
    });

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
