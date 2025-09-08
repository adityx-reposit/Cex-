use std::collections::HashMap;

use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{input::Side, orderbook};
#[derive(Serialize,Deserialize)]
pub struct Orderbook{
    pub bids:HashMap<u32,Vec<UserOrder>>,
    pub asks:HashMap<u32,Vec<UserOrder>>,
    pub order_id_index:u32
}


#[derive(Serialize,Deserialize)]
pub struct UserOrder{
    pub  user_id:u32,
    pub qty:u32,
    pub order_id:u32
}

impl Orderbook{
   pub fn new()->Self{
        Self{
        bids:HashMap::new(),
        asks:HashMap::new(),
        order_id_index:0
       } 
    }
}

impl Orderbook{
    pub fn Create_Order(&mut self,price:u32,quantity:u32,user_id:u32,side:Side){
        let order_id=self.order_id_index;
        self.order_id_index=self.order_id_index +1;
        if Side::Buy == side{
            self.bids.entry(price).or_insert(Vec::new()).push(UserOrder { user_id:90 , qty: quantity, order_id: 123 });
            
        }
        self.order_id_index+=1;
        if Side::Sell == side{
            self.asks.entry(price).or_insert(Vec::new()).push(UserOrder { user_id:90 , qty: quantity, order_id: 123 });
            
        }
    }
    pub fn DeleteOrder(){
       
    }

    // pub fn get_depth(&self)->&orderbook{
        
    // }
}