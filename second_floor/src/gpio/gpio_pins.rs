use crate::{gpio::gpio_async_interrupts, model::ParkingLot};
use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
use rust_socketio::client::Client;
use std::sync::{Arc, Mutex};

pub struct GpioPins {
    pub pass_through_sensor_1: InputPin,
    pub pass_through_sensor_2: InputPin,
    pub space_address_1: Arc<Mutex<OutputPin>>,
    pub space_address_2: Arc<Mutex<OutputPin>>,
    pub space_address_3: Arc<Mutex<OutputPin>>,
    pub space_sensor: Arc<Mutex<InputPin>>,
    pub closed_signal: Arc<Mutex<OutputPin>>,

    pub pass_through_sensor_1_level: Arc<Mutex<Level>>,
    pub pass_through_sensor_2_level: Arc<Mutex<Level>>,
}

impl GpioPins {
    pub fn new(gpio: &Gpio) -> Self {
        GpioPins {
            pass_through_sensor_1: gpio.get(0).unwrap().into_input_pulldown(),
            pass_through_sensor_2: gpio.get(7).unwrap().into_input_pulldown(),
            space_address_1: Arc::new(Mutex::new(gpio.get(9).unwrap().into_output_low())),
            space_address_2: Arc::new(Mutex::new(gpio.get(11).unwrap().into_output_low())),
            space_address_3: Arc::new(Mutex::new(gpio.get(15).unwrap().into_output_low())),
            space_sensor: Arc::new(Mutex::new(gpio.get(1).unwrap().into_input_pulldown())),
            closed_signal: Arc::new(Mutex::new(gpio.get(14).unwrap().into_output_low())),
            pass_through_sensor_1_level: Arc::new(Mutex::new(Level::Low)),
            pass_through_sensor_2_level: Arc::new(Mutex::new(Level::Low)),
        }
    }

    pub fn setup_interrupts(
        &mut self,
        client: &Arc<Mutex<Client>>,
        parking_lot: &Arc<Mutex<ParkingLot>>,
    ) {
        gpio_async_interrupts::configure(self, client, parking_lot)
    }
}
