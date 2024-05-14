mod model;

use crate::model::{Car, ParkingLot};
use ctrlc;
use rppal::gpio::{Gpio, Level, Trigger};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::{Arc, Mutex};

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
    let gpio = Gpio::new().unwrap();

    let mut entry_open_signal = gpio.get(23).unwrap().into_input_pulldown();
    let mut entry_close_signal = gpio.get(24).unwrap().into_input_pulldown();
    let entry_engine = Arc::new(Mutex::new(gpio.get(10).unwrap().into_output_low()));
    let mut exit_open_signal = gpio.get(25).unwrap().into_input_pulldown();
    let mut exit_close_signal = gpio.get(12).unwrap().into_input_pulldown();
    let exit_engine = Arc::new(Mutex::new(gpio.get(17).unwrap().into_output_low()));
    let space_address_1 = Arc::new(Mutex::new(gpio.get(22).unwrap().into_output_low()));
    let space_address_2 = Arc::new(Mutex::new(gpio.get(26).unwrap().into_output_low()));
    let space_address_3 = Arc::new(Mutex::new(gpio.get(19).unwrap().into_output_low()));
    let space_sensor = Arc::new(Mutex::new(gpio.get(18).unwrap().into_input_pulldown()));
    let closed_signal = Arc::new(Mutex::new(gpio.get(27).unwrap().into_output_low()));

    // Creating the parking lot
    let parking_lot = Arc::new(Mutex::new(ParkingLot::new()));

    // Callback to turn on the entry engine when the entry open signal is triggered
    let entry_engine_clone = entry_engine.clone();
    entry_open_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            entry_engine_clone.lock().unwrap().set_high()
        })
        .unwrap();

    // Callback to turn off the entry engine when the entry close signal is triggered
    let entry_engine_clone = entry_engine.clone();
    let parking_lot_clone = parking_lot.clone();
    let space_address_1_clone = space_address_1.clone();
    let space_address_2_clone = space_address_2.clone();
    let space_address_3_clone = space_address_3.clone();
    let space_sensor_clone = space_sensor.clone();
    entry_close_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            // Locking the resources
            let mut inner_entry_engine = entry_engine_clone.lock().unwrap();
            let mut inner_parking_lot = parking_lot_clone.lock().unwrap();
            let mut inner_space_address_1 = space_address_1_clone.lock().unwrap();
            let mut inner_space_address_2 = space_address_2_clone.lock().unwrap();
            let mut inner_space_address_3 = space_address_3_clone.lock().unwrap();
            let inner_space_sensor = space_sensor_clone.lock().unwrap();

            // Turn off the entry engine
            inner_entry_engine.set_low();

            // after the entry is closed, scan the parking lot to check if a car entered in a parking space
            let mut car_inserted: Option<Car> = None;

            for address in 0..8 {
                let (address_1, address_2, address_3) = convert_address_to_levels(address as u8);

                inner_space_address_1.write(address_1);
                inner_space_address_2.write(address_2);
                inner_space_address_3.write(address_3);

                let parking_space = &mut inner_parking_lot.spaces[address];

                // if the space is occupied and there was no car in the parking space database,
                // then the car entered in the parking space
                if inner_space_sensor.is_high() && parking_space.car.is_none() {
                    let car = Car::new();
                    parking_space.car = Some(car.clone());
                    car_inserted = Some(car);
                }
            }

            // if a car entered in a parking space, send a signal to the server
            if let Some(car) = car_inserted {
                // send signal to server
                println!("{:?}", serde_json::to_string(&car).unwrap());
            } else {
                println!("No car inserted");
            }
        })
        .unwrap();

    // Callback to turn on the exit engine when the exit open signal is triggered
    let exit_engine_clone = exit_engine.clone();
    exit_open_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            exit_engine_clone.lock().unwrap().set_high();
        })
        .unwrap();

    // Callback to turn off the exit engine when the exit close signal is triggered
    let exit_engine_clone = exit_engine.clone();
    let parking_lot_clone = parking_lot.clone();
    let space_address_1_clone = space_address_1.clone();
    let space_address_2_clone = space_address_2.clone();
    let space_address_3_clone = space_address_3.clone();
    let space_sensor_clone = space_sensor.clone();
    exit_close_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            // Locking the resources
            let mut inner_exit_engine = exit_engine_clone.lock().unwrap();
            let mut inner_parking_lot = parking_lot_clone.lock().unwrap();
            let mut inner_space_address_1 = space_address_1_clone.lock().unwrap();
            let mut inner_space_address_2 = space_address_2_clone.lock().unwrap();
            let mut inner_space_address_3 = space_address_3_clone.lock().unwrap();
            let inner_space_sensor = space_sensor_clone.lock().unwrap();

            // Turn off the exit engine
            inner_exit_engine.set_low();

            // after the exit is closed, scan the parking lot to check if a car exited from a parking space
            let mut car_removed: Option<Car> = None;

            for address in 0..8 {
                let (address_1, address_2, address_3) = convert_address_to_levels(address as u8);

                inner_space_address_1.write(address_1);
                inner_space_address_2.write(address_2);
                inner_space_address_3.write(address_3);

                let parking_space = &mut inner_parking_lot.spaces[address];

                // if the space is occupied and there was a car in the parking space database,
                // then the car exited from the parking space
                if inner_space_sensor.is_high() && parking_space.car.is_some() {
                    let car = parking_space.car.take().unwrap();
                    car_removed = Some(car);
                }
            }

            // if a car exited from a parking space, send a signal to the server
            if let Some(car) = car_removed {
                // send signal to server
                println!("{:?}", serde_json::to_string(&car).unwrap());
            } else {
                println!("No car removed");
            }
        })
        .unwrap();

    println!("Program running");

    // Waiting for Ctrl-C
    while running.load(SeqCst) {}

    println!("Program terminated");
}

fn convert_address_to_levels(address: u8) -> (Level, Level, Level) {
    let address_1 = if address & 0b001 != 0 {
        Level::High
    } else {
        Level::Low
    };

    let address_2 = if address & 0b010 != 0 {
        Level::High
    } else {
        Level::Low
    };

    let address_3 = if address & 0b100 != 0 {
        Level::High
    } else {
        Level::Low
    };

    (address_1, address_2, address_3)
}
