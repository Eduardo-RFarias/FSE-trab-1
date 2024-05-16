mod gpio;
mod model;
mod socket;
mod utils;

use crate::gpio::gpio_pins::GpioPins;
use crate::model::ParkingLot;
use crate::socket::socket_client;
use crate::utils::configure_graceful_shutdown;
use std::sync::atomic::Ordering::SeqCst;

fn main() {
    let running = configure_graceful_shutdown::get_running_flag();

    // Setting up GPIO pins
    let mut gpio_pins = GpioPins::new();

    // Creating the parking lot
    let parking_lot = ParkingLot::new();

    // Setting up the socket.io client
    let client = socket_client::new_client(&gpio_pins, &parking_lot);

    // Configuring the GPIO pins to handle interrupts
    gpio_pins.setup_interrupts(&client, &parking_lot);

    // Keep the program running until running turns false
    println!("Program started");
    while running.load(SeqCst) {}

    // Disconnecting the client
    client.lock().unwrap().disconnect().unwrap();
    println!("Program terminated");
}
