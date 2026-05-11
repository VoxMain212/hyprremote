use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod commands;
mod ws_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler::handle_socket))
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("🚀 Server running on http://{}", addr);
    
    axum::serve(listener, app).await.unwrap();
}