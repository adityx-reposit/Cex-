use std::sync::{Arc, Mutex};

use actix_web::{ web, App, HttpServer};
use env_logger::Env;

use crate::{orderbook::Orderbook, routes::{create_order, delete_order, get_depth}};

pub mod output;
pub mod routes;
pub mod input;
pub mod orderbook;



#[actix_web::main]
async fn main()->Result<(), std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
  let orderbook = Arc::new(Mutex::new(Orderbook::default()));
HttpServer::new(move || {
  App::new()
  .wrap(actix_web::middleware::Logger::default())
 .app_data(web::Data::new(orderbook.clone())) 
  .service(create_order)
  .service(delete_order)
  .service(get_depth)

 })
 .bind("127.0.0.1:3000")?
 .run()
 .await 
}

