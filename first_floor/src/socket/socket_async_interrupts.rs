use crate::gpio::gpio_pins::GpioPins;
use crate::model::ParkingLot;
use crate::socket::socket_operations::{CLOSING_FLOOR, OPENING_FLOOR, PARKING_LOT_STATE};
use rust_socketio::ClientBuilder;
use rust_socketio::Payload;
use std::sync::{Arc, Mutex};

pub fn set_close_floor_signal(client: ClientBuilder, gpio_pins: &GpioPins) -> ClientBuilder {
    let closed_signal_clone = gpio_pins.closed_signal.clone();

    client.on(CLOSING_FLOOR, move |_, _| {
        closed_signal_clone.lock().unwrap().set_high();
    })
}

pub fn set_open_floor_signal(client: ClientBuilder, gpio_pins: &GpioPins) -> ClientBuilder {
    let closed_signal_clone = gpio_pins.closed_signal.clone();

    client.on(OPENING_FLOOR, move |_, _| {
        closed_signal_clone.lock().unwrap().set_low();
    })
}

pub fn set_parking_lot_state_signal(
    client: ClientBuilder,
    parking_lot: &Arc<Mutex<ParkingLot>>,
) -> ClientBuilder {
    let parking_lot_clone = parking_lot.clone();

    client.on(PARKING_LOT_STATE, move |payload, _| {
        let spaces: Vec<bool>;

        if let Payload::Text(data) = payload {
            spaces = serde_json::from_str(&data[0].to_string()).unwrap();
        } else {
            panic!("Payload is not text");
        }

        parking_lot_clone.lock().unwrap().update_spaces(&spaces);
    })
}
