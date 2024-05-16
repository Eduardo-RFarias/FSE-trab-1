use crate::gpio::gpio_async_interrupts;
use crate::model::ParkingLot;
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rust_socketio::client::Client;
use std::sync::{Arc, Mutex};

pub struct GpioPins {
    pub entry_open_signal: InputPin,
    pub entry_close_signal: InputPin,
    pub entry_engine: Arc<Mutex<OutputPin>>,
    pub exit_open_signal: InputPin,
    pub exit_close_signal: InputPin,
    pub exit_engine: Arc<Mutex<OutputPin>>,
    pub space_address_1: Arc<Mutex<OutputPin>>,
    pub space_address_2: Arc<Mutex<OutputPin>>,
    pub space_address_3: Arc<Mutex<OutputPin>>,
    pub space_sensor: Arc<Mutex<InputPin>>,
    pub closed_signal: Arc<Mutex<OutputPin>>,
}

impl GpioPins {
    pub fn new() -> GpioPins {
        let gpio = Gpio::new().unwrap();

        GpioPins {
            entry_open_signal: gpio.get(23).unwrap().into_input_pulldown(),
            entry_close_signal: gpio.get(24).unwrap().into_input_pulldown(),
            entry_engine: Arc::new(Mutex::new(gpio.get(10).unwrap().into_output_low())),
            exit_open_signal: gpio.get(25).unwrap().into_input_pulldown(),
            exit_close_signal: gpio.get(12).unwrap().into_input_pulldown(),
            exit_engine: Arc::new(Mutex::new(gpio.get(17).unwrap().into_output_low())),
            space_address_1: Arc::new(Mutex::new(gpio.get(22).unwrap().into_output_low())),
            space_address_2: Arc::new(Mutex::new(gpio.get(26).unwrap().into_output_low())),
            space_address_3: Arc::new(Mutex::new(gpio.get(19).unwrap().into_output_low())),
            space_sensor: Arc::new(Mutex::new(gpio.get(18).unwrap().into_input_pulldown())),
            closed_signal: Arc::new(Mutex::new(gpio.get(27).unwrap().into_output_low())),
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
