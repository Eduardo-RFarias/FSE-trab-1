mod configure_graceful_shutdown;
mod gpio_async_interrupts;
mod gpio_pins;
mod model;
mod socket_client;
mod socket_operations;

use crate::gpio_pins::GpioPins;
use crate::model::ParkingLot;
use std::sync::atomic::Ordering::SeqCst;

fn main() {
    let running = configure_graceful_shutdown::get_running_flag();

    // Setting up GPIO pins
    let mut gpio_pins = GpioPins::new();

    // Setting up the socket.io client
    let client = socket_client::new_client(&gpio_pins);

    // Creating the parking lot
    let parking_lot = ParkingLot::new();

    // Configuring the GPIO pins to handle interrupts
    gpio_async_interrupts::configure(&mut gpio_pins, &client, &parking_lot);

    // Keep the program running until running turns false
    println!("Program started");
    while running.load(SeqCst) {}

    // Disconnecting the client
    client.lock().unwrap().disconnect().unwrap();
    println!("Program terminated");
}
