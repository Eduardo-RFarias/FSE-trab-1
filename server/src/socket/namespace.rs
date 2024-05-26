use super::handlers::{
    handle_car_arrived, handle_car_departed, handle_close_floor, handle_close_parking_lot,
    handle_disconnect, handle_open_floor, handle_open_parking_lot, handle_reset_database,
    save_connection, send_floor_state,
};
use crate::database::Database;
use socketioxide::{extract::SocketRef, SocketIo};

pub fn configure_socket_namespace(io: &SocketIo) {
    let database = Database::new();

    io.ns("/", move |socket: SocketRef| async move {
        let conn_was_saved = save_connection(&socket, &database).await;

        if conn_was_saved {
            // Send the parking lot state to the client that just connected (or reconnected)
            send_floor_state(&socket, &database).await;
        } else {
            socket.disconnect().unwrap();
            return;
        }

        handle_disconnect(&socket, database.clone());

        handle_car_arrived(&socket, database.clone());
        handle_car_departed(&socket, database.clone());

        handle_close_floor(&socket, database.clone());
        handle_close_parking_lot(&socket, database.clone());

        handle_open_parking_lot(&socket, database.clone());
        handle_open_floor(&socket, database.clone());

        handle_reset_database(&socket, database.clone());
    });
}
