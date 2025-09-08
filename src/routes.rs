use std::sync::{Arc, Mutex};

use actix_web::{
    delete, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};

use crate::{
    input::{CreateOrderInput, DeleteOrder, Side},
    orderbook::{Depth, DepthResponse, OpenOrder, Orderbook},
    output::{CreateOrderResponse, DeleteOrderResponse},
};

#[post("/order")]
pub async fn create_order(
    Json(body): Json<CreateOrderInput>,
    orderbook: web::Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();

    let order = OpenOrder {
        price: body.price,
        quantity: body.quantity,
        side: body.side.clone(),
        user_id: body.user_id.clone(),
        order_id: "".to_string(), // will be replaced inside `create_order`
        filled_quantity: 0,
    };

    orderbook.create_order(order);

    HttpResponse::Ok().json(CreateOrderResponse {
        userid: body.user_id,
    })
}

#[delete("/order")]
pub async fn delete_order(
    Json(body): Json<DeleteOrder>,
    orderbook: web::Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();
    orderbook.delete_order(body);

    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 0.0,
    })
}

#[get("/order")]
pub async fn get_depth(
    orderbook: web::Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let orderbook = orderbook.lock().unwrap();
    let depth = orderbook.get_depth();

    HttpResponse::Ok().json(depth)
}
