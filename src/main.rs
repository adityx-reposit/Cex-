use actix_web::{delete, get, post, App, HttpServer, Responder};

#[actix_web::main]
async fn main()->Result<(), std::io::Error> {
HttpServer::new(move || {
  App::new()
  .service(create_order)
  .service(delete_order)
  .service(get_depth)

 })
 .bind("127.0.0.1.8080")?
 .run()
 .await 
}

#[post("/order")]
async fn create_order()-> impl Responder{    
    "create order"
}
#[delete("/order")]
async fn delete_order()-> impl Responder{    
    "Delete order"
}
#[get("/order")]
async fn get_depth()-> impl Responder{    
    "getting depth "
}
