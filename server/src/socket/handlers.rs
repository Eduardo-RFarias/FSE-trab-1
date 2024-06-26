use super::{
    constants::{
        CAR_ARRIVED_EVENT, CAR_DEPARTED_EVENT, CLIENT_ID_HEADER, CLOSE_FLOOR_EVENT,
        CLOSE_PARKING_LOT_EVENT, FLOOR_STATE_EVENT, OPEN_FLOOR_EVENT, OPEN_PARKING_LOT_EVENT,
        PARKING_LOT_STATE_EVENT, RESET_DATABASE_EVENT,
    },
    payloads::ParkingSpaceModifiedPayload,
};
use crate::{database::Database, models::client::ClientId};
use socketioxide::extract::{Data, SocketRef};
use std::sync::{Arc, Mutex, MutexGuard};

pub async fn save_connection(socket: &SocketRef, database: &Arc<Mutex<Database>>) -> bool {
    // When a client connects, we need to check if it has a client_id header
    let client_id_header = socket.req_parts().headers.get(CLIENT_ID_HEADER);

    // If it does, we try to parse it to a ClientId enum
    if let Some(client_id_header) = client_id_header {
        let client_id_str = client_id_header.to_str().unwrap();

        // If the client_id is valid, we store it in the database and join the room
        if let Some(client_id) = ClientId::from_str(client_id_str) {
            println!("Client {} connected as {}", socket.id, client_id);

            database
                .lock()
                .unwrap()
                .clients
                .insert(socket.id.to_string(), client_id);

            socket.join(client_id.to_string()).unwrap();

            true
        }
        // If the client_id is invalid, return false
        else {
            println!(
                "Unidentified client {} tried to connect as {}",
                socket.id, client_id_str
            );

            false
        }
    }
    // If the client_id header is missing, return false
    else {
        println!("Unidentified client {} tried to connect", socket.id);

        false
    }
}

pub fn handle_disconnect(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    // When a client disconnects, if it was in the database, we remove it
    socket.on_disconnect(move |socket: SocketRef| async move {
        let sid = socket.id.to_string();

        if let Some(client_id) = database.lock().unwrap().clients.remove(&sid) {
            println!("Client {} disconnected from {}", sid, client_id);
        } else {
            println!("Client {} disconnected", sid);
        }
    });
}

pub async fn send_floor_state(socket: &SocketRef, database: &Arc<Mutex<Database>>) {
    let database = database.lock().unwrap();
    let client_id = database.clients.get(&socket.id.to_string()).unwrap();
    let floor_number = client_id.to_index();
    let floor = database.get_floor(floor_number).unwrap();

    socket
        .within(client_id.to_string())
        .emit(FLOOR_STATE_EVENT, vec![floor.as_bool_vec()])
        .unwrap();

    if *client_id == ClientId::GroundFloor && database.parking_lot_is_full().unwrap() {
        socket
            .within(client_id.to_string())
            .emit(CLOSE_PARKING_LOT_EVENT, ())
            .unwrap();
    } else if database.floor_is_full(floor_number).unwrap() {
        socket
            .within(client_id.to_string())
            .emit(CLOSE_FLOOR_EVENT, ())
            .unwrap();
    }

    send_parking_lot_state(socket, &database);
}

fn send_parking_lot_state(socket: &SocketRef, database: &MutexGuard<Database>) {
    let parking_lot = database.get_parking_lot_state().unwrap();

    socket
        .within(ClientId::App.to_string())
        .emit(PARKING_LOT_STATE_EVENT, parking_lot)
        .unwrap();
}

pub fn handle_car_arrived(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        CAR_ARRIVED_EVENT,
        move |socket: SocketRef, Data(payload): Data<ParkingSpaceModifiedPayload>| async move {
            let mut database = database.lock().unwrap();
            let client_id = *database.clients.get(&socket.id.to_string()).unwrap();
            let floor_number = client_id.to_index();

            // park the new car in the respective floor and parking space
            database
                .park_vehicle(payload.timestamp, floor_number, payload.parking_space)
                .unwrap();

            // if the floor filled up, close the floor
            if database.floor_is_full(floor_number).unwrap() {
                database.close_floor(floor_number).unwrap();

                socket
                    .within(client_id.to_string())
                    .emit(CLOSE_FLOOR_EVENT, ())
                    .unwrap();
            }

            // if the parking lot filled up, close the parking lot
            if database.parking_lot_is_full().unwrap() {
                database.close_parking_lot().unwrap();

                socket
                    .within(ClientId::GroundFloor.to_string())
                    .emit(CLOSE_PARKING_LOT_EVENT, ())
                    .unwrap();
            }

            // send the new floor state to the client
            send_parking_lot_state(&socket, &database);
        },
    );
}

pub fn handle_car_departed(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        CAR_DEPARTED_EVENT,
        move |socket: SocketRef, Data(payload): Data<ParkingSpaceModifiedPayload>| async move {
            let mut database = database.lock().unwrap();

            // If the parking lot is full, open it
            if database.parking_lot_is_full().unwrap() {
                database.open_parking_lot().unwrap();

                socket
                    .within(ClientId::GroundFloor.to_string())
                    .emit(OPEN_PARKING_LOT_EVENT, ())
                    .unwrap();
            }

            // If the floor is full, open it
            let client_id = *database.clients.get(&socket.id.to_string()).unwrap();
            let floor_number = client_id.to_index();

            if database.floor_is_full(floor_number).unwrap() {
                database.open_floor(floor_number).unwrap();

                socket
                    .within(client_id.to_string())
                    .emit(OPEN_FLOOR_EVENT, ())
                    .unwrap();
            }

            // Remove the vehicle from the parking space
            database
                .unpark_vehicle(floor_number, payload.parking_space, payload.timestamp)
                .unwrap();

            // Send the new floor state to the client
            send_parking_lot_state(&socket, &database);
        },
    )
}

pub fn handle_close_parking_lot(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        CLOSE_PARKING_LOT_EVENT,
        move |socket: SocketRef| async move {
            let mut database = database.lock().unwrap();

            database.close_parking_lot().unwrap();

            socket
                .within(ClientId::GroundFloor.to_string())
                .emit(CLOSE_PARKING_LOT_EVENT, ())
                .unwrap();

            send_parking_lot_state(&socket, &database);
        },
    );
}

pub fn handle_close_floor(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        CLOSE_FLOOR_EVENT,
        move |socket: SocketRef, Data(floor_number): Data<i32>| async move {
            let floor = ClientId::from_index(floor_number).unwrap();
            let mut database = database.lock().unwrap();

            database.close_floor(floor_number).unwrap();

            socket
                .within(floor.to_string())
                .emit(CLOSE_FLOOR_EVENT, ())
                .unwrap();

            send_parking_lot_state(&socket, &database);
        },
    );
}

pub fn handle_open_parking_lot(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        OPEN_PARKING_LOT_EVENT,
        move |socket: SocketRef| async move {
            let mut database = database.lock().unwrap();

            database.open_parking_lot().unwrap();

            socket
                .within(ClientId::GroundFloor.to_string())
                .emit(OPEN_PARKING_LOT_EVENT, ())
                .unwrap();

            send_parking_lot_state(&socket, &database);
        },
    );
}

pub fn handle_open_floor(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        OPEN_FLOOR_EVENT,
        move |socket: SocketRef, Data(floor_number): Data<i32>| async move {
            let floor = ClientId::from_index(floor_number).unwrap();
            let mut database = database.lock().unwrap();

            database.open_floor(floor_number).unwrap();

            socket
                .within(floor.to_string())
                .emit(OPEN_FLOOR_EVENT, ())
                .unwrap();

            send_parking_lot_state(&socket, &database);
        },
    );
}

pub fn handle_reset_database(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(RESET_DATABASE_EVENT, move |socket: SocketRef| async move {
        let mut database = database.lock().unwrap();

        // Reset the database
        database.reset_parking_lot().unwrap();

        // For each floor
        for client_id in ClientId::iter_floors() {
            // Emit the parking lot state event with all parking spaces empty
            socket
                .within(client_id.to_string())
                .emit(FLOOR_STATE_EVENT, vec![false; 8])
                .unwrap();

            // After that, emit the open floor event to open the floor
            socket
                .within(client_id.to_string())
                .emit(OPEN_FLOOR_EVENT, ())
                .unwrap();
        }

        // Emit the open parking lot event to open the parking lot
        socket
            .within(ClientId::GroundFloor.to_string())
            .emit(OPEN_PARKING_LOT_EVENT, ())
            .unwrap();

        // Send the new parking lot state to the client
        send_parking_lot_state(&socket, &database);
    });
}
