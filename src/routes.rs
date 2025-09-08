
use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web::{self, Data, Json}, HttpResponse, Responder};


use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::{self, Orderbook}, output::{ CreateOrderResponse, DeleteOrderResponse, DepthResponse}}; 



#[post("/order")]
pub async fn create_order(
    Json(body): Json<CreateOrderInput>,
    orderbook: web::Data<Arc<Mutex<Orderbook>>>
) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();

    let price = body.price;
    let quantity = body.quantity;
    let user_id = body.user_id;
    let side = body.side;

    orderbook.Create_Order(price, quantity, user_id, side);

    HttpResponse::Ok().json(CreateOrderResponse {
        userid: body.user_id,
    })
}

#[delete("/order")]
pub async fn delete_order(Json(body):Json<DeleteOrder>)-> impl Responder{    
  let order_Id = body.order_id;
  
   return  HttpResponse::Ok().json(DeleteOrderResponse{
      filled_qty:321,
      average_price:23.54
   });
}
#[get("/order")]
pub async fn get_depth(order:web::Data<Orderbook>)-> impl Responder{    
 let response = get_depth(order);


    return HttpResponse::Ok().json(DepthResponse{
      bids,
    asks,
        lastUpdateId:String::from("adityayadav")
      

    })
}
