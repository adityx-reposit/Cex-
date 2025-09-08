use serde::{Deserialize,Serialize};

use crate::orderbook::Depth;


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
   pub    bids:Vec<(Depth)>,
   pub    asks:Vec<(Depth)>,
   pub    lastUpdateId:String
}