use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::input::{DeleteOrder, Side};
use crate::database::DatabaseManager;


#[derive(Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    pub price: u32,
    pub quantity: u32,
    pub side: Side,
    pub user_id: u32,
    pub order_id: String,
    pub filled_quantity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub buyer_order_id: String,
    pub seller_order_id: String,
    pub price: u32,
    pub quantity: u32,
    pub buyer_user_id: u32,
    pub seller_user_id: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Serialize, Deserialize)]
pub struct Orderbook {  
    pub bids: HashMap<String, Vec<OpenOrder>>,
    pub asks: HashMap<String, Vec<OpenOrder>>,
    pub order_id_index: u64,
    #[serde(skip)]
    pub db_manager: DatabaseManager,
    pub trade_count: u64, // For snapshot triggers
}

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Depth {
    pub price: u32,
    pub quantity: u32,
}

#[derive( Serialize, Deserialize)]
pub struct DepthResponse {
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}

impl Default for Orderbook {
    fn default() -> Self {
        Self {
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_id_index: 0,
            db_manager: DatabaseManager::new(),
            trade_count: 0,
        }
    }
}

impl Orderbook {
    pub fn create_order(&mut self, order: OpenOrder) -> Vec<Trade> {
        let order_id = self.order_id_index.to_string();
        self.order_id_index += 1;
        
        let mut executed_trades = Vec::new();
        
        match order.side {
            Side::Buy => {
                // Try to match with existing sell orders
                executed_trades = self.match_buy_order(&order_id, order.user_id, order.price, order.quantity);
                
                // If there's remaining quantity, add to bids
                let remaining_qty = order.quantity - executed_trades.iter().map(|t| t.quantity).sum::<u32>();
                if remaining_qty > 0 {
                    let open_order = OpenOrder {
                        price: order.price,
                        quantity: remaining_qty,
                        side: order.side,
                        user_id: order.user_id,
                        order_id: order_id.clone(),
                        filled_quantity: order.quantity - remaining_qty,
                    };
                    self.bids.entry(order.price.to_string()).or_insert(Vec::new()).push(open_order);
                    
                    // Save order to database
                    let _ = self.db_manager.save_order(
                        &order_id,
                        order.user_id as i32,
                        "Buy",
                        order.price as i32,
                        order.quantity as i32,
                        (order.quantity - remaining_qty) as i32,
                        if remaining_qty == order.quantity { "Active" } else { "PartiallyFilled" }
                    );
                }
            }
            Side::Sell => {
                // Try to match with existing buy orders
                executed_trades = self.match_sell_order(&order_id, order.user_id, order.price, order.quantity);
                
                // If there's remaining quantity, add to asks
                let remaining_qty = order.quantity - executed_trades.iter().map(|t| t.quantity).sum::<u32>();
                if remaining_qty > 0 {
                    let open_order = OpenOrder {
                        price: order.price,
                        quantity: remaining_qty,
                        side: order.side,
                        user_id: order.user_id,
                        order_id: order_id.clone(),
                        filled_quantity: order.quantity - remaining_qty,
                    };
                    self.asks.entry(order.price.to_string()).or_insert(Vec::new()).push(open_order);
                    
                    // Save order to database
                    let _ = self.db_manager.save_order(
                        &order_id,
                        order.user_id as i32,
                        "Sell",
                        order.price as i32,
                        order.quantity as i32,
                        (order.quantity - remaining_qty) as i32,
                        if remaining_qty == order.quantity { "Active" } else { "PartiallyFilled" }
                    );
                }
            }
        }
        
        // Save all executed trades to database
        for trade in &executed_trades {
            let _ = self.db_manager.save_trade(
                &trade.trade_id,
                &trade.buyer_order_id,
                &trade.seller_order_id,
                trade.price as i32,
                trade.quantity as i32,
                trade.buyer_user_id as i32,
                trade.seller_user_id as i32,
            );
        }
        
        self.trade_count += executed_trades.len() as u64;
        
        // Create snapshot every 10 trades or if this is a significant trade
        if self.trade_count % 10 == 0 || executed_trades.len() > 0 {
            let _ = self.create_snapshot();
        }
        
        executed_trades
    }

    pub fn delete_order(&mut self, order: DeleteOrder) {
        // Find and remove from bids
        if let Some(price) = self.bids.iter().find_map(|(price, orders)| {
            if orders.iter().any(|o| o.order_id == order.order_id) {
                Some(price.clone())
            } else {
                None
            }
        }) {
            if let Some(orders) = self.bids.get_mut(&price) {
                orders.retain(|o| o.order_id != order.order_id);
            }
        }

        // Find and remove from asks
        if let Some(price) = self.asks.iter().find_map(|(price, orders)| {
            if orders.iter().any(|o| o.order_id == order.order_id) {
                Some(price.clone())
            } else {
                None
            }
        }) {
            if let Some(orders) = self.asks.get_mut(&price) {
                orders.retain(|o| o.order_id != order.order_id);
            }
        }
    }

    pub fn get_depth(&self) -> DepthResponse {
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        for (price, orders) in self.bids.iter() {
            bids.push(Depth { price: price.parse().unwrap(), quantity: orders.iter().map(|o| o.quantity).sum() });
        }
        for (price, orders) in self.asks.iter() {
            asks.push(Depth { price: price.parse().unwrap(), quantity: orders.iter().map(|o| o.quantity).sum() });  
        }
        DepthResponse { bids, asks }
    }

    // Order matching methods
    fn match_buy_order(&mut self, order_id: &str, user_id: u32, price: u32, quantity: u32) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut remaining_qty = quantity;
        
        // Sort asks by price (ascending - best price first)
        let mut ask_prices: Vec<u32> = self.asks.keys()
            .filter(|p| p.parse::<u32>().unwrap() <= price)
            .map(|p| p.parse().unwrap())
            .collect();
        ask_prices.sort();
        
        for ask_price in ask_prices {
            if remaining_qty == 0 { break; }
            
            let price_key = ask_price.to_string();
            if let Some(orders) = self.asks.get_mut(&price_key) {
                let mut i = 0;
                while i < orders.len() && remaining_qty > 0 {
                    let trade_qty = std::cmp::min(remaining_qty, orders[i].quantity);
                    
                    // Create trade
                    let trade = Trade {
                        trade_id: Uuid::new_v4().to_string(),
                        buyer_order_id: order_id.to_string(),
                        seller_order_id: orders[i].order_id.clone(),
                        price: ask_price,
                        quantity: trade_qty,
                        buyer_user_id: user_id,
                        seller_user_id: orders[i].user_id,
                        created_at: Utc::now(),
                    };
                    trades.push(trade.clone());
                    
                    // Update quantities
                    remaining_qty -= trade_qty;
                    orders[i].quantity -= trade_qty;
                    orders[i].filled_quantity += trade_qty;
                    
                    // Update database
                    let new_status = if orders[i].quantity == 0 { "Filled" } else { "PartiallyFilled" };
                    let _ = self.db_manager.update_order(&orders[i].order_id, orders[i].filled_quantity as i32, new_status);
                    
                    // Remove order if fully filled
                    if orders[i].quantity == 0 {
                        orders.remove(i);
                    } else {
                        i += 1;
                    }
                }
                
                // Remove empty price levels
                if orders.is_empty() {
                    self.asks.remove(&price_key);
                }
            }
        }
        
        trades
    }

    fn match_sell_order(&mut self, order_id: &str, user_id: u32, price: u32, quantity: u32) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut remaining_qty = quantity;
        
        // Sort bids by price (descending - best price first)
        let mut bid_prices: Vec<u32> = self.bids.keys()
            .filter(|p| p.parse::<u32>().unwrap() >= price)
            .map(|p| p.parse().unwrap())
            .collect();
        bid_prices.sort_by(|a, b| b.cmp(a));
        
        for bid_price in bid_prices {
            if remaining_qty == 0 { break; }
            
            let price_key = bid_price.to_string();
            if let Some(orders) = self.bids.get_mut(&price_key) {
                let mut i = 0;
                while i < orders.len() && remaining_qty > 0 {
                    let trade_qty = std::cmp::min(remaining_qty, orders[i].quantity);
                    
                    // Create trade
                    let trade = Trade {
                        trade_id: Uuid::new_v4().to_string(),
                        buyer_order_id: orders[i].order_id.clone(),
                        seller_order_id: order_id.to_string(),
                        price: bid_price,
                        quantity: trade_qty,
                        buyer_user_id: orders[i].user_id,
                        seller_user_id: user_id,
                        created_at: Utc::now(),
                    };
                    trades.push(trade.clone());
                    
                    // Update quantities
                    remaining_qty -= trade_qty;
                    orders[i].quantity -= trade_qty;
                    orders[i].filled_quantity += trade_qty;
                    
                    // Update database
                    let new_status = if orders[i].quantity == 0 { "Filled" } else { "PartiallyFilled" };
                    let _ = self.db_manager.update_order(&orders[i].order_id, orders[i].filled_quantity as i32, new_status);
                    
                    // Remove order if fully filled
                    if orders[i].quantity == 0 {
                        orders.remove(i);
                    } else {
                        i += 1;
                    }
                }
                
                // Remove empty price levels
                if orders.is_empty() {
                    self.bids.remove(&price_key);
                }
            }
        }
        
        trades
    }

    // Snapshot functionality
    pub fn create_snapshot(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let snapshot_state = serde_json::to_string(&self)?;
        let snapshot_id = Uuid::new_v4().to_string();
        
        self.db_manager.save_snapshot(&snapshot_id, &snapshot_state)?;
        println!("Snapshot created: {}", snapshot_id);
        Ok(())
    }

    pub fn load_from_snapshot(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(snapshot) = self.db_manager.get_latest_snapshot()? {
            let restored_orderbook: Orderbook = serde_json::from_str(&snapshot.orderbook_state)?;
            self.bids = restored_orderbook.bids;
            self.asks = restored_orderbook.asks;
            self.order_id_index = restored_orderbook.order_id_index;
            self.trade_count = restored_orderbook.trade_count;
            println!("Orderbook restored from snapshot: {}", snapshot.snapshot_id);
        }
        Ok(())
    }

    pub fn get_trade_history(&mut self, limit: i64) -> Result<Vec<crate::database::Trade>, Box<dyn std::error::Error>> {
        Ok(self.db_manager.get_trades(limit)?)
    }
}