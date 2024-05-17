mod database;
mod models;
mod socket;

use socket::{namespace, server};
use socketioxide::SocketIo;

#[tokio::main]
async fn main() {
    let (layer, io) = SocketIo::new_layer();

    // Configure the one and only namespace of the socket.io server
    namespace::configure_socket_namespace(&io);

    // Configure the axum server and run it, this will block the main thread
    server::configure_axum_server(layer).await;
}
