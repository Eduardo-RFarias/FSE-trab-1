use crate::gpio::gpio_pins::GpioPins;
use crate::model::{ParkingLot, ParkingSpaceModifiedPayload};
use crate::socket::socket_operations::{CAR_ARRIVED, CAR_DEPARTED};
use chrono::Utc;
use rppal::gpio::{Level, Trigger};
use rust_socketio::client::Client;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn configure(
    gpio_pins: &mut GpioPins,
    client: &Arc<Mutex<Client>>,
    parking_lot: &Arc<Mutex<ParkingLot>>,
) {
    // Configure the entry open signal for when a car enters the parking lot
    configure_entry_open_signal(gpio_pins);

    // Configure the entry close signal for when a car passes the entry gate
    configure_entry_close_signal(gpio_pins, parking_lot, client);

    // Configure the exit open signal for when a car leaves the parking lot
    configure_exit_open_signal(gpio_pins, parking_lot, client);

    // Configure the exit close signal for when a car passes the exit gate
    configure_exit_close_signal(gpio_pins);
}

fn configure_entry_open_signal(gpio_pins: &mut GpioPins) {
    let entry_engine_clone = gpio_pins.entry_engine.clone();

    gpio_pins
        .entry_open_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            entry_engine_clone.lock().unwrap().set_high()
        })
        .unwrap();
}

fn configure_entry_close_signal(
    gpio_pins: &mut GpioPins,
    parking_lot: &Arc<Mutex<ParkingLot>>,
    client: &Arc<Mutex<Client>>,
) {
    let parking_lot_clone = parking_lot.clone();
    let client_clone = client.clone();
    let entry_engine_clone = gpio_pins.entry_engine.clone();
    let space_address_1_clone = gpio_pins.space_address_1.clone();
    let space_address_2_clone = gpio_pins.space_address_2.clone();
    let space_address_3_clone = gpio_pins.space_address_3.clone();
    let space_sensor_clone = gpio_pins.space_sensor.clone();

    gpio_pins
        .entry_close_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            // Turn off the entry engine
            entry_engine_clone.lock().unwrap().set_low();

            // Record the time when the car entered in the parking lot
            let car_entered_in = Utc::now().timestamp();

            // Wait for the car to find a parking space
            thread::sleep(Duration::from_secs(5));

            // after the entry is closed, scan the parking lot to check if a car entered in a parking space
            let mut parking_space_occupied = -1;

            for address in 0..8 {
                let (address_1, address_2, address_3) = convert_address_to_levels(address as u8);

                space_address_1_clone.lock().unwrap().write(address_1);
                space_address_2_clone.lock().unwrap().write(address_2);
                space_address_3_clone.lock().unwrap().write(address_3);

                // Wait the sensor to stabilize
                thread::sleep(Duration::from_millis(50));

                let parking_lot_spaces = &mut parking_lot_clone.lock().unwrap().spaces;

                // if the space is occupied and there was no car in the parking space database,
                // then the car entered in the parking space
                if space_sensor_clone.lock().unwrap().is_high()
                    && parking_lot_spaces[address] == false
                {
                    parking_lot_spaces[address] = true;
                    parking_space_occupied = address as i32;
                    break;
                }
            }

            // if a car entered in a parking space, send a signal to the server
            if parking_space_occupied != -1 {
                client_clone
                    .lock()
                    .unwrap()
                    .emit(
                        CAR_ARRIVED,
                        ParkingSpaceModifiedPayload {
                            parking_space: parking_space_occupied,
                            timestamp: car_entered_in,
                        },
                    )
                    .unwrap();
            }
        })
        .unwrap();
}

fn configure_exit_open_signal(
    gpio_pins: &mut GpioPins,
    parking_lot: &Arc<Mutex<ParkingLot>>,
    client: &Arc<Mutex<Client>>,
) {
    let parking_lot_clone = parking_lot.clone();
    let client_clone = client.clone();
    let exit_engine_clone = gpio_pins.exit_engine.clone();
    let space_address_1_clone = gpio_pins.space_address_1.clone();
    let space_address_2_clone = gpio_pins.space_address_2.clone();
    let space_address_3_clone = gpio_pins.space_address_3.clone();
    let space_sensor_clone = gpio_pins.space_sensor.clone();

    gpio_pins
        .exit_open_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            // Turn on the exit engine
            exit_engine_clone.lock().unwrap().set_high();

            // Record the time when the car left the parking lot
            let car_left_in = Utc::now().timestamp();

            // after the exit is opened, scan the parking lot to check if a car left a parking space
            let mut parking_space_liberated = -1;

            for address in 0..8 {
                let (address_1, address_2, address_3) = convert_address_to_levels(address as u8);

                space_address_1_clone.lock().unwrap().write(address_1);
                space_address_2_clone.lock().unwrap().write(address_2);
                space_address_3_clone.lock().unwrap().write(address_3);

                // Wait the sensor to stabilize
                thread::sleep(Duration::from_millis(50));

                let parking_lot_spaces = &mut parking_lot_clone.lock().unwrap().spaces;

                // if the space is empty and there was a car in the parking space database,
                // then the car left the parking space
                if space_sensor_clone.lock().unwrap().is_low()
                    && parking_lot_spaces[address] == true
                {
                    parking_lot_spaces[address] = false;
                    parking_space_liberated = address as i32;
                    break;
                }
            }

            // if a car left the parking space, send a signal to the server
            if parking_space_liberated != -1 {
                client_clone
                    .lock()
                    .unwrap()
                    .emit(
                        CAR_DEPARTED,
                        ParkingSpaceModifiedPayload {
                            parking_space: parking_space_liberated,
                            timestamp: car_left_in,
                        },
                    )
                    .unwrap();
            }
        })
        .unwrap();
}

fn configure_exit_close_signal(gpio_pins: &mut GpioPins) {
    let exit_engine_clone = gpio_pins.exit_engine.clone();

    gpio_pins
        .exit_close_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            exit_engine_clone.lock().unwrap().set_low();
        })
        .unwrap();
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
