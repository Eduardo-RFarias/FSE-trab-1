use super::handlers::{
    handle_car_arrived, handle_car_departed, handle_disconnect, save_connection,
    send_parking_lot_state,
};
use crate::database::Database;
use socketioxide::{extract::SocketRef, SocketIo};

pub fn configure_socket_namespace(io: &SocketIo) {
    let database = Database::new();

    io.ns("/", move |socket: SocketRef| async move {
        let conn_was_saved = save_connection(&socket, database.clone()).await;

        if conn_was_saved {
            send_parking_lot_state(&socket, database.clone()).await;
        } else {
            socket.disconnect().unwrap();
            return;
        }

        handle_disconnect(&socket, database.clone());
        handle_car_arrived(&socket, database.clone());
        handle_car_departed(&socket, database.clone());
    });
}
