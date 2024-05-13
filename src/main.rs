mod access_management;

use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::{Arc, Mutex};

use ctrlc;
use rppal::gpio::{Gpio, Trigger};

use access_management::register_access_management_handlers;

fn main() {
    // Handling Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Received Ctrl-C, shutting down...");
        r.store(false, SeqCst);
    })
    .unwrap();

    // Setting up GPIO
    let gpio = Gpio::new().unwrap();

    // Call the function to handle the opening/exit barriers
    register_access_management_handlers(&gpio);

    //let space_address_1 = Arc::new(Mutex::new(gpio.get(22).unwrap().into_output_low()));
    //let space_address_2 = Arc::new(Mutex::new(gpio.get(26).unwrap().into_output_low()));
    //let space_address_3 = Arc::new(Mutex::new(gpio.get(19).unwrap().into_output_low()));

    //let space_sensor_1 = Arc::new(Mutex::new(gpio.get(18).unwrap().into_input_pulldown()));

    //let closed_signal = Arc::new(Mutex::new(gpio.get(27).unwrap().into_output_low()));


    // Waiting for Ctrl-C
    while running.load(SeqCst) {}

    println!("Program terminated");
}
