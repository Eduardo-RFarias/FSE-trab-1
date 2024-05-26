use termion::raw::RawTerminal;

use crate::{menus, models::ParkingLotDataPayload};
use std::{
    io::Stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn set(
    stdout: Arc<Mutex<RawTerminal<Stdout>>>,
    parking_lot: Arc<Mutex<ParkingLotDataPayload>>,
) {
    thread::spawn(move || loop {
        {
            let mut stdout = stdout.lock().unwrap();
            let parking_lot = parking_lot.lock().unwrap();
            menus::dashboard(&mut stdout, &parking_lot);
        }
        thread::sleep(Duration::from_secs(5));
    });
}
