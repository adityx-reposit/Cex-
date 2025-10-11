use rusqlite::Connection;
use std::path::Path;

pub fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "cex.db";
    
    // Check if database already exists
    if Path::new(db_path).exists() {
        println!("Database already exists, skipping initialization");
        return Ok(());
    }

    // Create the database file by establishing connection
    let connection = Connection::open(db_path)?;
    
    // Create tables
    create_tables(&connection)?;

    println!("Database initialized successfully");
    Ok(())
}

fn create_tables(connection: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Create trades table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS trades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            trade_id TEXT NOT NULL UNIQUE,
            buyer_order_id TEXT NOT NULL,
            seller_order_id TEXT NOT NULL,
            price INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            buyer_user_id INTEGER NOT NULL,
            seller_user_id INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create orders table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id TEXT NOT NULL UNIQUE,
            user_id INTEGER NOT NULL,
            side TEXT NOT NULL,
            price INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            filled_quantity INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'Active',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create snapshots table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS snapshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            snapshot_id TEXT NOT NULL UNIQUE,
            orderbook_state TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create indexes for better performance
    connection.execute("CREATE INDEX IF NOT EXISTS idx_trades_created_at ON trades(created_at)", [])?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_trades_price ON trades(price)", [])?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status)", [])?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id)", [])?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_orders_price ON orders(price)", [])?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_snapshots_created_at ON snapshots(created_at)", [])?;

    Ok(())
}