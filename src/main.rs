use actix_web::{delete, get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::routes::{create_order, delete_order, get_depth};

pub mod output;
pub mod routes;
pub mod input;



#[actix_web::main]
async fn main()->Result<(), std::io::Error> {
HttpServer::new(move || {
  App::new()
  .service(create_order)
  .service(delete_order)
  .service(get_depth)

 })
 .bind("127.0.0.1:3000")?
 .run()
 .await 
}

