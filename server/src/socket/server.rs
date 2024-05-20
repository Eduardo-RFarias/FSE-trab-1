use crate::socket::constants::SERVER_ADDRESS;
use axum::{self, Router};
use socketioxide::layer::SocketIoLayer;
use tokio::{net::TcpListener, signal};

pub async fn configure_axum_server(layer: SocketIoLayer) {
    let app = Router::new().layer(layer);
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    let shutdown = async {
        signal::ctrl_c().await.unwrap();
        println!("Shutting down...");
    };

    println!("Server running on: http://{}", SERVER_ADDRESS);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
        .unwrap();
}
