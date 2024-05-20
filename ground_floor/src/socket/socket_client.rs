use crate::gpio::gpio_pins::GpioPins;
use crate::model::ParkingLot;
use crate::socket::socket_async_interrupts::{
    set_close_parking_lot_signal, set_open_parking_lot_signal, set_parking_lot_state_signal,
};
use crate::socket::socket_operations::{CLIENT_HEADER, SERVER_URL};
use rust_socketio::{client::Client, ClientBuilder};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn new_client(
    gpio_pins: &GpioPins,
    parking_lot: &Arc<Mutex<ParkingLot>>,
) -> Arc<Mutex<Client>> {
    // Creating the client
    let mut client = ClientBuilder::new(SERVER_URL)
        .opening_header(CLIENT_HEADER.key, CLIENT_HEADER.value)
        .reconnect_on_disconnect(true)
        .max_reconnect_attempts(10);

    // Setting up the close parking lot signal
    client = set_close_parking_lot_signal(client, gpio_pins);

    // Setting up the open parking lot signal
    client = set_open_parking_lot_signal(client, gpio_pins);

    // Setting up the parking lot state signal
    client = set_parking_lot_state_signal(client, parking_lot);

    // Connecting to the server
    for _ in 0..10 {
        let connection = client.clone();

        match connection.connect() {
            Ok(connection) => {
                return Arc::new(Mutex::new(connection));
            }
            Err(_) => {
                println!("Error connecting to the server. Retrying...");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    panic!("Could not connect to the server");
}
