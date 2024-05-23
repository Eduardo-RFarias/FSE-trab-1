mod constants;

use crate::constants::{
    CLIENT_HEADER, CLOSE_FLOOR_EVENT, CLOSE_PARKING_LOT_EVENT, OPEN_FLOOR_EVENT,
    OPEN_PARKING_LOT_EVENT, RESET_DATABASE_EVENT, SERVER_ADDRESS,
};
use ctrlc;
use rust_socketio::{client::Client, ClientBuilder};
use serde_json::json;
use std::{
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use text_io::read;

macro_rules! clear {
    () => {
        std::process::Command::new("clear").status().unwrap();
    };
}

macro_rules! pause {
    () => {
        println!("\nPress Enter to continue...");
        let _: String = read!("{}\n");
    };
}

fn main() {
    let client_builder = ClientBuilder::new(SERVER_ADDRESS)
        .opening_header(CLIENT_HEADER.key, CLIENT_HEADER.value)
        .reconnect_on_disconnect(true)
        .max_reconnect_attempts(10);

    let mut connection: Option<Client> = None;

    for _ in 0..10 {
        match client_builder.clone().connect() {
            Ok(conn) => {
                connection = Some(conn);
                break;
            }
            Err(_) => {
                println!("Error connecting to the server. Retrying...");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    if connection.is_none() {
        println!("Failed to connect to the server. Exiting...");
        process::exit(1);
    }

    let client = Arc::new(Mutex::new(connection.unwrap()));

    setup_ctrlc_handler(client.clone());

    loop {
        clear!();

        println!("Test program\n");
        println!("1. Close parking lot");
        println!("2. Close floor");
        println!("3. Open parking lot");
        println!("4. Open floor");
        println!("5. Reset database");
        println!("0. Exit");

        let choice: i32 = read!();

        match choice {
            1 => close_parking_lot(client.clone()),
            2 => close_floor(client.clone()),
            3 => open_parking_lot(client.clone()),
            4 => open_floor(client.clone()),
            5 => reset_database(client.clone()),
            0 => break,
            _ => {
                println!("Invalid choice, please try again.");
                pause!();
            }
        }
    }

    println!("Program exited successfully!")
}

fn close_parking_lot(socket_client: Arc<Mutex<Client>>) {
    clear!();

    socket_client
        .lock()
        .unwrap()
        .emit(CLOSE_PARKING_LOT_EVENT, json!(()))
        .unwrap();

    println!("Sent close parking lot event to server.");
    pause!();
}

fn close_floor(socket_client: Arc<Mutex<Client>>) {
    clear!();

    println!("Which floor do you want to close?");
    println!("1. Ground floor");
    println!("2. First floor");
    println!("3. Second floor");
    println!("0. Cancel");

    let mut floor: i32;

    loop {
        floor = read!();

        match floor {
            1 | 2 | 3 => break,
            0 => return,
            _ => println!("Invalid floor number, please try again."),
        }
    }

    socket_client
        .lock()
        .unwrap()
        .emit(CLOSE_FLOOR_EVENT, json!(floor))
        .unwrap();

    println!("Sent close floor {} to server.", floor);
    pause!();
}

fn open_parking_lot(socket_client: Arc<Mutex<Client>>) {
    clear!();

    socket_client
        .lock()
        .unwrap()
        .emit(OPEN_PARKING_LOT_EVENT, json!(()))
        .unwrap();

    println!("Sent open parking lot event to server.");
    pause!();
}

fn open_floor(socket_client: Arc<Mutex<Client>>) {
    clear!();

    println!("Which floor do you want to open?");
    println!("1. Ground floor");
    println!("2. First floor");
    println!("3. Second floor");

    let mut floor: i32;

    loop {
        floor = read!();

        match floor {
            1 | 2 | 3 => {
                break;
            }
            _ => {
                println!("Invalid floor number, please try again.");
            }
        }
    }

    socket_client
        .lock()
        .unwrap()
        .emit(OPEN_FLOOR_EVENT, json!(floor))
        .unwrap();

    println!("Sent open floor {} to server.", floor);
    pause!();
}

fn reset_database(socket_client: Arc<Mutex<Client>>) {
    clear!();

    socket_client
        .lock()
        .unwrap()
        .emit(RESET_DATABASE_EVENT, json!(()))
        .unwrap();

    println!("Sent reset database event to server.");
    pause!();
}

fn setup_ctrlc_handler(socket_client: Arc<Mutex<Client>>) {
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C signal, exiting...");

        if let Ok(client) = socket_client.lock() {
            if let Ok(_) = client.disconnect() {
                process::exit(0);
            }
        }

        process::exit(1);
    })
    .unwrap();
}
