mod gpio_pins;

use crate::gpio_pins::GpioPins;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Arc;

fn main() {
    // Handling Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Received Ctrl-C, shutting down...");
        r.store(false, SeqCst);
    })
    .unwrap();

    // Setting up GPIO pins
    let mut gpio_pins = GpioPins::new();

    // Registering access management handlers
    gpio_pins.register_access_management_handlers();

    println!("Program running");

    // Waiting for Ctrl-C
    while running.load(SeqCst) {}

    println!("Program terminated");
}
