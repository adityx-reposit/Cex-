use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::env;

// Database connection
pub fn establish_connection() -> Connection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "cex.db".to_string());
    Connection::open(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Database models
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub id: i32,
    pub trade_id: String,
    pub buyer_order_id: String,
    pub seller_order_id: String,
    pub price: i32,
    pub quantity: i32,
    pub buyer_user_id: i32,
    pub seller_user_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: i32,
    pub order_id: String,
    pub user_id: i32,
    pub side: String, // "Buy" or "Sell"
    pub price: i32,
    pub quantity: i32,
    pub filled_quantity: i32,
    pub status: String, // "Active", "Filled", "Cancelled", "PartiallyFilled"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Snapshot {
    pub id: i32,
    pub snapshot_id: String,
    pub orderbook_state: String, // JSON serialized orderbook
    pub created_at: DateTime<Utc>,
}

// Database operations
pub struct DatabaseManager {
    connection: Connection,
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            connection: establish_connection(),
        }
    }

    pub fn save_trade(&mut self, trade_id: &str, buyer_order_id: &str, seller_order_id: &str, 
                     price: i32, quantity: i32, buyer_user_id: i32, seller_user_id: i32) -> Result<()> {
        self.connection.execute(
            "INSERT INTO trades (trade_id, buyer_order_id, seller_order_id, price, quantity, buyer_user_id, seller_user_id, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![trade_id, buyer_order_id, seller_order_id, price, quantity, buyer_user_id, seller_user_id, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn save_order(&mut self, order_id: &str, user_id: i32, side: &str, price: i32, 
                     quantity: i32, filled_quantity: i32, status: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO orders (order_id, user_id, side, price, quantity, filled_quantity, status, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![order_id, user_id, side, price, quantity, filled_quantity, status, Utc::now().to_rfc3339(), Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn update_order(&mut self, order_id: &str, filled_qty: i32, status: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE orders SET filled_quantity = ?1, status = ?2, updated_at = ?3 WHERE order_id = ?4",
            params![filled_qty, status, Utc::now().to_rfc3339(), order_id],
        )?;
        Ok(())
    }

    pub fn get_active_orders(&mut self) -> Result<Vec<Order>> {
        let mut stmt = self.connection.prepare("SELECT id, order_id, user_id, side, price, quantity, filled_quantity, status, created_at, updated_at FROM orders WHERE status = 'Active'")?;
        let order_iter = stmt.query_map([], |row| {
            Ok(Order {
                id: row.get(0)?,
                order_id: row.get(1)?,
                user_id: row.get(2)?,
                side: row.get(3)?,
                price: row.get(4)?,
                quantity: row.get(5)?,
                filled_quantity: row.get(6)?,
                status: row.get(7)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?).unwrap().with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?).unwrap().with_timezone(&Utc),
            })
        })?;

        let mut orders = Vec::new();
        for order in order_iter {
            orders.push(order?);
        }
        Ok(orders)
    }

    pub fn save_snapshot(&mut self, snapshot_id: &str, orderbook_state: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO snapshots (snapshot_id, orderbook_state, created_at) VALUES (?1, ?2, ?3)",
            params![snapshot_id, orderbook_state, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn get_latest_snapshot(&mut self) -> Result<Option<Snapshot>> {
        let mut stmt = self.connection.prepare("SELECT id, snapshot_id, orderbook_state, created_at FROM snapshots ORDER BY created_at DESC LIMIT 1")?;
        let mut rows = stmt.query_map([], |row| {
            Ok(Snapshot {
                id: row.get(0)?,
                snapshot_id: row.get(1)?,
                orderbook_state: row.get(2)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?).unwrap().with_timezone(&Utc),
            })
        })?;

        if let Some(snapshot) = rows.next() {
            Ok(Some(snapshot?))
        } else {
            Ok(None)
        }
    }

    pub fn get_trades(&mut self, limit: i64) -> Result<Vec<Trade>> {
        let mut stmt = self.connection.prepare("SELECT id, trade_id, buyer_order_id, seller_order_id, price, quantity, buyer_user_id, seller_user_id, created_at FROM trades ORDER BY created_at DESC LIMIT ?1")?;
        let trade_iter = stmt.query_map(params![limit], |row| {
            Ok(Trade {
                id: row.get(0)?,
                trade_id: row.get(1)?,
                buyer_order_id: row.get(2)?,
                seller_order_id: row.get(3)?,
                price: row.get(4)?,
                quantity: row.get(5)?,
                buyer_user_id: row.get(6)?,
                seller_user_id: row.get(7)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?).unwrap().with_timezone(&Utc),
            })
        })?;

        let mut trades = Vec::new();
        for trade in trade_iter {
            trades.push(trade?);
        }
        Ok(trades)
    }
}