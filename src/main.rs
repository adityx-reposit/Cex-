use std::sync::{Arc, Mutex};

use actix_web::{ web, App, HttpServer};
use env_logger::Env;

use crate::{orderbook::Orderbook, routes::{create_order, delete_order, get_depth, get_trades}};

pub mod output;
pub mod routes;
pub mod input;
pub mod orderbook;
pub mod database;
pub mod db_init;




#[actix_web::main]
async fn main()->Result<(), std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    
    // Initialize database
    if let Err(e) = crate::db_init::initialize_database() {
        eprintln!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }
    
    // Create orderbook and load from snapshot if available
    let mut orderbook = Orderbook::default();
    if let Err(e) = orderbook.load_from_snapshot() {
        eprintln!("Warning: Could not load from snapshot: {}", e);
    }
    
    let orderbook = Arc::new(Mutex::new(orderbook));
    
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(orderbook.clone())) 
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
            .service(get_trades)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await 
}

