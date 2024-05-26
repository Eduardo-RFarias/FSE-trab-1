mod constants;
mod ctrlc_handler;
mod dashboard_pooling;
mod menus;
mod models;
mod operations;
mod socket_client;

use crate::models::ParkingLotDataPayload;
use std::{
    io::{stdin, stdout},
    sync::{Arc, Mutex},
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let stdin = stdin();
    let stdout = Arc::new(Mutex::new(stdout().into_raw_mode().unwrap()));

    let parking_lot = Arc::new(Mutex::new(ParkingLotDataPayload::new()));

    let client = socket_client::create(stdout.clone(), parking_lot.clone());

    dashboard_pooling::set(stdout.clone(), parking_lot.clone());
    ctrlc_handler::set(client.clone(), stdout.clone());

    menus::init_console(&stdout);
    menus::main_menu(&stdout);

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('0') => {
                break;
            }
            Key::Ctrl('c') => {
                break;
            }
            Key::Char('1') => {
                operations::close_parking_lot(&client, &stdout);
            }
            Key::Char('2') => {
                operations::close_floor(&client, &stdout);
            }
            Key::Char('3') => {
                operations::open_parking_lot(&client, &stdout);
            }
            Key::Char('4') => {
                operations::open_floor(&client, &stdout);
            }
            Key::Char('5') => {
                operations::reset_database(&client, &stdout);
            }
            _ => {}
        }
    }

    menus::finalize_console(&stdout);
}
