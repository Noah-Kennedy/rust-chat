#[macro_use]
extern crate log;

use actix_web::{HttpServer, App, web, middleware::Logger};

use log::LevelFilter;

use std::io;

use crate::handlers::*;

/// Contains domain model for program
mod model;

/// Contains request handlers
mod handlers;

/// Contains websocket handler code
mod ws;

/// Contains a crate-specific error type.
/// I didnt really use it much.
mod errors;

/// Random utility functions
mod util;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // read env vars from file
    dotenv::dotenv().unwrap();

    std::env::set_var("RUST_LOG", "actix_web=debug");

    // start logging
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    // initialize our global state
    let state = web::Data::new(AppState::new().await);

    info!("Launching server!");

    HttpServer::new(move ||
        App::new()
            // add logging middleware
            .wrap(Logger::default())

            // add state middleware
            .app_data(state.clone())

            // add http get routes
            .route("/users", web::get().to(get_users))
            .route("/messages", web::get().to(get_messages))
            .route("/ws", web::get().to(get_websocket))

            // add http post routes
            .route("/users", web::post().to(add_user))
            .route("/messages", web::post().to(send_message))
    )
        .bind("127.0.0.1:6969")?
        .run()
        .await
}
