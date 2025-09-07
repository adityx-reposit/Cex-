use serde::{Deserialize,Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct CreateOrderResponse{
    pub userid:u32
}


#[derive(Debug,Serialize,Deserialize)]
pub struct DeleteOrderResponse{
    pub filled_qty:u32,
    pub average_price:f32,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct DepthResponse {
   pub    bids:Vec<(f64,f64)>,
   pub    asks:Vec<(f64,f64)>,
   pub    lastUpdateId:String
}