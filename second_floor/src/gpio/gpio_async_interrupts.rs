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
    // Configure the pass-through sensor 1, for when a car passes through
    // the sensor at the start of the corridor
    configure_pass_through_sensor_1(gpio_pins, parking_lot, client);

    // Configure the pass-through sensor 2, for when a car passes through
    // the sensor at the end of the corridor
    configure_pass_through_sensor_2(gpio_pins, parking_lot, client);
}

fn configure_pass_through_sensor_1(
    gpio_pins: &mut GpioPins,
    parking_lot: &Arc<Mutex<ParkingLot>>,
    client: &Arc<Mutex<Client>>,
) {
    let space_address_1_clone = gpio_pins.space_address_1.clone();
    let space_address_2_clone = gpio_pins.space_address_2.clone();
    let space_address_3_clone = gpio_pins.space_address_3.clone();
    let space_sensor_clone = gpio_pins.space_sensor.clone();
    let pass_through_sensor_1_level_clone = gpio_pins.pass_through_sensor_1_level.clone();
    let pass_through_sensor_2_level_clone = gpio_pins.pass_through_sensor_2_level.clone();
    let parking_lot_clone = parking_lot.clone();
    let client_clone = client.clone();
    gpio_pins
        .pass_through_sensor_1
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            // If the sensor 1 is triggered before sensor 2, then the car yet to pass
            // through the corridor. Just set sensor 1 flag to High
            if *(pass_through_sensor_2_level_clone.lock().unwrap()) == Level::Low {
                *(pass_through_sensor_1_level_clone.lock().unwrap()) = Level::High;
                return;
            }

            // If the sensor 1 is triggered after sensor 2, then the car has passed
            // through the corridor. Set sensor flags to Low and notify the server that
            // the car has exited the floor
            *(pass_through_sensor_1_level_clone.lock().unwrap()) = Level::Low;
            *(pass_through_sensor_2_level_clone.lock().unwrap()) = Level::Low;

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

fn configure_pass_through_sensor_2(
    gpio_pins: &mut GpioPins,
    parking_lot: &Arc<Mutex<ParkingLot>>,
    client: &Arc<Mutex<Client>>,
) {
    let space_address_1_clone = gpio_pins.space_address_1.clone();
    let space_address_2_clone = gpio_pins.space_address_2.clone();
    let space_address_3_clone = gpio_pins.space_address_3.clone();
    let space_sensor_clone = gpio_pins.space_sensor.clone();
    let pass_through_sensor_1_level_clone = gpio_pins.pass_through_sensor_1_level.clone();
    let pass_through_sensor_2_level_clone = gpio_pins.pass_through_sensor_2_level.clone();
    let parking_lot_clone = parking_lot.clone();
    let client_clone = client.clone();
    gpio_pins
        .pass_through_sensor_2
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            // If the sensor 2 is triggered before sensor 1, then the car yet to pass
            // through the corridor. Just set sensor 2 flag to High
            if *(pass_through_sensor_1_level_clone.lock().unwrap()) == Level::Low {
                *(pass_through_sensor_2_level_clone.lock().unwrap()) = Level::High;
                return;
            }

            // If the sensor 2 is triggered after sensor 1, then the car has passed
            // through the corridor. Set sensor flags to Low and notify the server that
            // the car has entered the floor
            *(pass_through_sensor_1_level_clone.lock().unwrap()) = Level::Low;
            *(pass_through_sensor_2_level_clone.lock().unwrap()) = Level::Low;

            // Record the time when the car entered the parking lot
            let car_entered_in = Utc::now().timestamp();

            // Wait for the car to find a parking space
            thread::sleep(Duration::from_secs(3));

            // after the entry is opened, scan the parking lot to check if a car entered a parking space
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
                // then the car entered the parking space
                if space_sensor_clone.lock().unwrap().is_high()
                    && parking_lot_spaces[address] == false
                {
                    parking_lot_spaces[address] = true;
                    parking_space_occupied = address as i32;
                    break;
                }
            }

            // if a car entered the parking space, send a signal to the server
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
