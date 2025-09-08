use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Debug)]
pub struct CreateOrderInput{
   pub price :u32,
   pub quantity:u32,
   pub user_id :u32,
   pub side :Side
}


#[derive(Deserialize,Serialize,Debug,PartialEq)]
pub enum Side{
    Buy,
    Sell
}

#[derive(Serialize,Deserialize)]
pub struct DeleteOrder{
   pub order_id:String
}