use super::{
    constants::{
        CAR_ARRIVED_EVENT, CAR_DEPARTED_EVENT, CLIENT_ID_HEADER, CLOSE_FLOOR_EVENT,
        CLOSE_PARKING_LOT_EVENT, OPEN_FLOOR_EVENT, OPEN_PARKING_LOT_EVENT, PARKING_LOT_STATE_EVENT,
    },
    payloads::ParkingSpaceModifiedPayload,
};
use crate::{
    database::Database,
    models::{client::ClientId, parking_lot::Vehicle},
};
use socketioxide::extract::{Data, SocketRef};
use std::sync::{atomic::Ordering::SeqCst, Arc, Mutex};

pub async fn save_connection(socket: &SocketRef, database: Arc<Mutex<Database>>) -> bool {
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

pub async fn send_parking_lot_state(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    let database = database.lock().unwrap();
    let client_id = database.clients.get(&socket.id.to_string()).unwrap();
    let floor = &database.parking_lot.floors[client_id.to_index()];

    socket
        .within(client_id.to_string())
        .emit(PARKING_LOT_STATE_EVENT, vec![floor.as_bool_vec()])
        .unwrap();

    if *client_id == ClientId::GroundFloor && database.parking_lot.is_full() {
        socket
            .within(client_id.to_string())
            .emit(CLOSE_PARKING_LOT_EVENT, ())
            .unwrap();
    } else if floor.is_full() {
        socket
            .within(client_id.to_string())
            .emit(CLOSE_FLOOR_EVENT, ())
            .unwrap();
    }
}

pub fn handle_car_arrived(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        CAR_ARRIVED_EVENT,
        move |socket: SocketRef, Data(payload): Data<ParkingSpaceModifiedPayload>| async move {
            let mut database = database.lock().unwrap();

            // create a new vehicle instance
            let vehicle = Vehicle {
                id: database.id_counter.fetch_add(1, SeqCst),
                entry_time: payload.timestamp,
            };

            // park the new car in the respective floor and parking space
            let client_id = *database.clients.get(&socket.id.to_string()).unwrap();
            let floor = &mut database.parking_lot.floors[client_id.to_index()];

            floor.spots[payload.parking_space as usize].park(vehicle);

            // if the floor filled up, close the floor
            if floor.is_full() {
                socket
                    .within(client_id.to_string())
                    .emit(CLOSE_FLOOR_EVENT, ())
                    .unwrap();
            }

            // if the parking lot filled up, close the parking lot
            if database.parking_lot.is_full() {
                socket
                    .within(ClientId::GroundFloor.to_string())
                    .emit(CLOSE_PARKING_LOT_EVENT, ())
                    .unwrap();
            }
        },
    );
}

pub fn handle_car_departed(socket: &SocketRef, database: Arc<Mutex<Database>>) {
    socket.on(
        CAR_DEPARTED_EVENT,
        move |socket: SocketRef, Data(payload): Data<ParkingSpaceModifiedPayload>| async move {
            let mut database = database.lock().unwrap();

            // If the parking lot is full, open it
            if database.parking_lot.is_full() {
                socket
                    .within(ClientId::GroundFloor.to_string())
                    .emit(OPEN_PARKING_LOT_EVENT, ())
                    .unwrap();
            }

            // If the floor is full, open it
            let client_id = *database.clients.get(&socket.id.to_string()).unwrap();
            let floor = &mut database.parking_lot.floors[client_id.to_index()];

            if floor.is_full() {
                socket
                    .within(client_id.to_string())
                    .emit(OPEN_FLOOR_EVENT, ())
                    .unwrap();
            }

            // Remove the vehicle from the parking space and calculate the fee
            let vehicle = floor.spots[payload.parking_space as usize].unpark();

            if let Some(vehicle) = vehicle {
                let fee = vehicle.calculate_fee(payload.timestamp);

                // Emit the fee to the client
                println!("Vehicle {} paid a fee of {}", vehicle.id, fee);
            } else {
                panic!(
                    "Client {} tried to depart a vehicle from empty parking space {}",
                    client_id, payload.parking_space
                );
            }
        },
    )
}
