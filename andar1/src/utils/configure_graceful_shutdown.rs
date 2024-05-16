use ctrlc;
use std::{
    panic,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

pub fn get_running_flag() -> Arc<AtomicBool> {
    // Boolean to control the program execution, if false the program will stop
    let running = Arc::new(AtomicBool::new(true));

    // Handling Ctrl-C
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Received Ctrl-C, shutting down...");
        r.store(false, SeqCst);
    })
    .unwrap();

    // Handling panic
    let panic_hook = panic::take_hook();
    let r = running.clone();
    panic::set_hook(Box::new(move |panic_info| {
        panic_hook(panic_info);
        r.store(false, SeqCst);
    }));

    running
}
