
use actix_web::{delete, get, post, web::Json, HttpResponse, Responder};


use crate::{input::{CreateOrderInput, DeleteOrder}, output::{ CreateOrderResponse, DeleteOrderResponse, DepthResponse}}; 



#[post("/order")]
pub async fn create_order(body:Json<CreateOrderInput>)-> impl Responder{    
    let price =body.0.price;
    let quantity =body.0.quantity;
    let user_id =body.0.user_id;
    let side =body.0.side;
  
    return HttpResponse::Ok().json(CreateOrderResponse{
        userid:body.0.user_id
    });
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
pub async fn get_depth()-> impl Responder{    
     let bids = vec![
        (100.5, 10.0),  // price 100.5, qty 10
        (99.8, 5.0),
    ];

    let asks = vec![
        (101.2, 3.0),   // price 101.2, qty 3
        (102.0, 8.0),
    ];


    return HttpResponse::Ok().json(DepthResponse{
      bids,
    asks,
        lastUpdateId:String::from("adityayadav")
      

    })
}
