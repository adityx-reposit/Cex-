# CEX - Centralized Exchange Orderbook

A Rust-based centralized exchange orderbook system with database persistence and crash recovery capabilities.

## Features

- **Order Management**: Create and delete buy/sell orders
- **Trade Execution**: Automatic order matching with price-time priority
- **Database Persistence**: SQLite database for trades, orders, and snapshots
- **Crash Recovery**: Automatic snapshot creation and restoration
- **REST API**: HTTP endpoints for order operations and trade history

## Architecture

### Components

1. **Orderbook**: In-memory order matching engine
2. **Database**: SQLite with Diesel ORM for persistence
3. **Snapshot System**: Periodic orderbook state snapshots for recovery
4. **REST API**: Actix-web based HTTP server

### Database Schema

- **trades**: Executed trade records
- **orders**: Order history and status
- **snapshots**: Orderbook state snapshots for recovery

## API Endpoints

### POST /order

Create a new order

```json
{
  "price": 100,
  "quantity": 10,
  "user_id": 1,
  "side": "Buy"
}
```

Response includes executed trades if any matches occurred.

### DELETE /order

Cancel an existing order

```json
{
  "order_id": "123"
}
```

### GET /order

Get current orderbook depth (bids and asks)

### GET /trades

Get trade history

Query parameters:

- `limit`: Number of trades to return (default: 100)

## Order Matching Logic

1. **Buy Orders**: Match with existing sell orders at or below the buy price
2. **Sell Orders**: Match with existing buy orders at or above the sell price
3. **Price Priority**: Better prices execute first
4. **Time Priority**: Earlier orders at the same price execute first

## Snapshot System

- **Automatic Snapshots**: Created every 10 trades or on significant trades
- **Crash Recovery**: Latest snapshot loaded on server startup
- **State Persistence**: Complete orderbook state saved to database

## Running the System

1. **Install Dependencies**:

   ```bash
   cargo build
   ```

2. **Start Server**:

   ```bash
   cargo run
   ```

3. **Test with Sample Orders**:
   ```bash
   chmod +x test_orders.sh
   ./test_orders.sh
   ```

## Database

sql and postgres are added 


The system uses SQLite with the following tables:

- `trades`: Stores executed trades with buyer/seller information
- `orders`: Stores order history and current status
- `snapshots`: Stores periodic orderbook state snapshots

Database file: `cex.db` (created automatically)

## Error Handling

- Database connection errors are logged and handled gracefully
- Invalid orders return appropriate HTTP error codes
- Snapshot creation failures are logged but don't stop trading

## Performance Considerations

- In-memory orderbook for fast matching
- Database writes are asynchronous where possible
- Indexed database queries for efficient trade history retrieval
- Periodic snapshots balance recovery time vs. performance

## Frontend Trading Interface

A professional, dark-themed React trading interface is included in the `frontend/` directory.

### Features

- **Professional Design**: Dark gradient theme with glassmorphism effects
- **Real-time Updates**: Live orderbook and trade data
- **Smooth Animations**: Framer Motion powered transitions
- **Responsive Layout**: Optimized for desktop trading
- **Modern UI**: Inspired by Binance and Mango Markets

### Tech Stack

- React 18 with hooks
- Tailwind CSS for styling
- Framer Motion for animations
- Chart.js for price visualization
- Axios for API communication

### Quick Start

```bash
cd frontend
npm install
npm start
```

## Running the Complete System

### Backend Only

```bash
cargo run
```

### Full Trading Interface

```powershell
# Windows - starts both backend and frontend
.\start_trading_system.ps1
```

Access at:

- Backend API: http://127.0.0.1:3000
- Frontend UI: http://127.0.0.1:3001

## Development

### Adding New Features

1. Update database schema in `migrations/`
2. Add corresponding models in `database.rs`
3. Update orderbook logic in `orderbook.rs`
4. Add new API endpoints in `routes.rs`
5. Update frontend components as needed

### Testing

Use the provided `test_orders.sh` script or create custom tests with curl/Postman.

## Dependencies

### Backend

- **actix-web**: HTTP server framework
- **rusqlite**: SQLite database
- **serde**: Serialization
- **uuid**: Unique identifiers
- **chrono**: Timestamp handling

### Frontend

- **react**: UI framework
- **tailwindcss**: CSS framework
- **framer-motion**: Animation library
- **chart.js**: Chart visualization
- **axios**: HTTP client
