use rppal::gpio::{Gpio, InputPin, OutputPin, Trigger};
use std::sync::{Arc, Mutex};

pub struct GpioPins {
    pub entry_open_signal: InputPin,
    pub entry_close_signal: InputPin,
    pub entry_engine: Arc<Mutex<OutputPin>>,

    pub exit_open_signal: InputPin,
    pub exit_close_signal: InputPin,
    pub exit_engine: Arc<Mutex<OutputPin>>,

    pub space_address_1: OutputPin,
    pub space_address_2: OutputPin,
    pub space_address_3: OutputPin,

    pub space_sensor_1: InputPin,

    pub closed_signal: Arc<Mutex<OutputPin>>,
}

impl GpioPins {
    pub fn new() -> GpioPins {
        let gpio = Gpio::new().unwrap();

        // All the GPIO pins are initialized here
        GpioPins {
            entry_open_signal: gpio.get(23).unwrap().into_input_pulldown(),
            entry_close_signal: gpio.get(24).unwrap().into_input_pulldown(),
            entry_engine: Arc::new(Mutex::new(gpio.get(10).unwrap().into_output_low())),

            exit_open_signal: gpio.get(25).unwrap().into_input_pulldown(),
            exit_close_signal: gpio.get(12).unwrap().into_input_pulldown(),
            exit_engine: Arc::new(Mutex::new(gpio.get(17).unwrap().into_output_low())),

            space_address_1: gpio.get(22).unwrap().into_output_low(),
            space_address_2: gpio.get(26).unwrap().into_output_low(),
            space_address_3: gpio.get(19).unwrap().into_output_low(),

            space_sensor_1: gpio.get(18).unwrap().into_input_pulldown(),

            closed_signal: Arc::new(Mutex::new(gpio.get(27).unwrap().into_output_low())),
        }
    }

    pub fn register_access_management_handlers(&mut self) {
        // Callback to turn on the entry engine when the entry open signal is triggered
        let entry_engine_clone = self.entry_engine.clone();
        self.entry_open_signal
            .set_async_interrupt(Trigger::RisingEdge, move |_| {
                entry_engine_clone.lock().unwrap().set_high()
            })
            .unwrap();

        // Callback to turn off the entry engine when the entry close signal is triggered
        let entry_engine_clone = self.entry_engine.clone();
        self.entry_close_signal
            .set_async_interrupt(Trigger::RisingEdge, move |_| {
                entry_engine_clone.lock().unwrap().set_low();
            })
            .unwrap();

        // Callback to turn on the exit engine when the exit open signal is triggered
        let exit_engine_clone = self.exit_engine.clone();
        self.exit_open_signal
            .set_async_interrupt(Trigger::RisingEdge, move |_| {
                exit_engine_clone.lock().unwrap().set_high();
            })
            .unwrap();

        // Callback to turn off the exit engine when the exit close signal is triggered
        let exit_engine_clone = self.exit_engine.clone();
        self.exit_close_signal
            .set_async_interrupt(Trigger::RisingEdge, move |_| {
                exit_engine_clone.lock().unwrap().set_low();
            })
            .unwrap();
    }
}
