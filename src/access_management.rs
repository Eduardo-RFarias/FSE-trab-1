use std::sync::{Arc, Mutex};

use rppal::gpio::{Gpio, Trigger};

pub fn register_access_management_handlers(gpio: &Gpio) {
    let mut entry_open_signal = gpio.get(23).unwrap().into_input_pulldown();
    let mut entry_close_signal = gpio.get(24).unwrap().into_input_pulldown();
    let entry_engine = Arc::new(Mutex::new(gpio.get(10).unwrap().into_output_low()));

    let mut exit_open_signal = gpio.get(25).unwrap().into_input_pulldown();
    let mut exit_close_signal = gpio.get(12).unwrap().into_input_pulldown();
    let exit_engine = Arc::new(Mutex::new(gpio.get(17).unwrap().into_output_low()));

    let entry_engine_clone = entry_engine.clone();
    let exit_engine_clone = exit_engine.clone();

    entry_open_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            println!("Entry open signal triggered, setting entry engine high");

            entry_engine_clone.lock().unwrap().set_high();
        })
        .unwrap();

    entry_close_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            println!("Entry close signal triggered, setting entry engine low");

            entry_engine.lock().unwrap().set_low();
        })
        .unwrap();

    exit_open_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            println!("Exit open signal triggered, setting exit engine high");

            exit_engine_clone.lock().unwrap().set_high();
        })
        .unwrap();

    exit_close_signal
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            println!("Exit close signal triggered, setting exit engine low");

            exit_engine.lock().unwrap().set_low();
        })
        .unwrap();
}
