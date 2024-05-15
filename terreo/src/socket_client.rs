use crate::gpio_pins::GpioPins;
use crate::socket_operations::{
    CLIENT_HEADER, CLOSING_PARKING_LOT, OPENING_PARKING_LOT, SERVER_URL,
};
use rust_socketio::{client::Client, ClientBuilder};
use std::sync::{Arc, Mutex};

pub fn new_client(gpio_pins: &GpioPins) -> Arc<Mutex<Client>> {
    // Creating the client
    let mut client = ClientBuilder::new(SERVER_URL)
        .opening_header(CLIENT_HEADER.key, CLIENT_HEADER.value)
        .reconnect_on_disconnect(true)
        .max_reconnect_attempts(10);

    // Setting up the close parking lot signal
    client = set_close_parking_lot_signal(client, &gpio_pins);

    // Setting up the open parking lot signal
    client = set_open_parking_lot_signal(client, &gpio_pins);

    // Connecting to the server
    let client = client.connect().unwrap();

    // Returning the client
    Arc::new(Mutex::new(client))
}

fn set_close_parking_lot_signal(client: ClientBuilder, gpio_pins: &GpioPins) -> ClientBuilder {
    let closed_signal_clone = gpio_pins.closed_signal.clone();

    client.on(CLOSING_PARKING_LOT, move |_, _| {
        closed_signal_clone.lock().unwrap().set_high();
    })
}

fn set_open_parking_lot_signal(client: ClientBuilder, gpio_pins: &GpioPins) -> ClientBuilder {
    let closed_signal_clone = gpio_pins.closed_signal.clone();

    client.on(OPENING_PARKING_LOT, move |_, _| {
        closed_signal_clone.lock().unwrap().set_low();
    })
}
