mod utils;
mod routers;
mod views;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = utils::config::get_host().unwrap_or_else(|_| String::from("localhost"));
    let port = utils::config::get_port().unwrap_or_else(|_| 8080);

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .configure(routers::general_router::config_general_routes)
    })
    .bind((host, port.clone()))?
    .run()
    .await
}
