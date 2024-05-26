use crate::{
    constants::{
        CLOSE_FLOOR_EVENT, CLOSE_PARKING_LOT_EVENT, OPEN_FLOOR_EVENT, OPEN_PARKING_LOT_EVENT,
        RESET_DATABASE_EVENT,
    },
    menus,
};
use rust_socketio::client::Client;
use serde_json::json;
use std::{
    io::{stdin, Stdout},
    sync::{Arc, Mutex},
};
use termion::{event::Key, input::TermRead, raw::RawTerminal};

pub fn close_parking_lot(client: &Arc<Mutex<Client>>, stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    client
        .lock()
        .unwrap()
        .emit(CLOSE_PARKING_LOT_EVENT, json!(()))
        .unwrap();

    menus::feedback(stdout, "Ordem de fechamento de estacionamento enviada.");
}

pub fn close_floor(client: &Arc<Mutex<Client>>, stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    menus::floor_menu(stdout);

    let mut choice: i8 = -1;
    let stdin = stdin().lock();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('1') => {
                choice = 1;
                break;
            }
            Key::Char('2') => {
                choice = 2;
                break;
            }
            Key::Char('0') => {
                break;
            }
            _ => {}
        }
    }

    if choice != -1 {
        client
            .lock()
            .unwrap()
            .emit(CLOSE_FLOOR_EVENT, json!(choice))
            .unwrap();
    }

    menus::main_menu(stdout);

    if choice != -1 {
        menus::feedback(stdout, "Ordem de fechamento de andar enviada.");
    }
}

pub fn open_parking_lot(client: &Arc<Mutex<Client>>, stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    client
        .lock()
        .unwrap()
        .emit(OPEN_PARKING_LOT_EVENT, json!(()))
        .unwrap();

    menus::feedback(stdout, "Ordem de abertura de estacionamento enviada.");
}

pub fn open_floor(client: &Arc<Mutex<Client>>, stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    menus::floor_menu(stdout);

    let mut choice: i8 = -1;
    let stdin = stdin().lock();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('1') => {
                choice = 1;
                break;
            }
            Key::Char('2') => {
                choice = 2;
                break;
            }
            Key::Char('0') => {
                break;
            }
            _ => {}
        }
    }

    if choice != -1 {
        client
            .lock()
            .unwrap()
            .emit(OPEN_FLOOR_EVENT, json!(choice))
            .unwrap();
    }

    menus::main_menu(stdout);

    if choice != -1 {
        menus::feedback(stdout, "Ordem de abertura de andar enviada.");
    }
}

pub fn reset_database(client: &Arc<Mutex<Client>>, stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    client
        .lock()
        .unwrap()
        .emit(RESET_DATABASE_EVENT, json!(()))
        .unwrap();

    menus::feedback(stdout, "Ordem de reset de dados enviada.");
}
