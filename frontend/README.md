# CEX Trading Interface

A professional, dark-themed trading interface built with React, Tailwind CSS, and Framer Motion.

## Features

- **Professional Design**: Dark gradient theme with glassmorphism effects
- **Real-time Updates**: Live orderbook and trade data
- **Smooth Animations**: Framer Motion powered transitions
- **Responsive Layout**: Optimized for desktop trading
- **Modern UI**: Inspired by Binance and Mango Markets

## Tech Stack

- **React 18**: Modern React with hooks
- **Tailwind CSS**: Utility-first CSS framework
- **Framer Motion**: Animation library for smooth transitions
- **Chart.js**: Price chart visualization
- **Axios**: HTTP client for API communication
- **Lucide React**: Beautiful icon library

## Components

### Core Components

- `TradingInterface.js` - Main layout and state management
- `OrderForm.js` - Buy/sell order form with advanced options
- `OrderBook.js` - Real-time orderbook with animated depth bars
- `PriceChart.js` - Interactive price chart with Chart.js
- `RecentTrades.js` - Live trades feed with animations
- `ActiveOrders.js` - User's active orders with progress tracking
- `Tabs.js` - Tab navigation with smooth transitions

### Features

- **Order Form**: Buy/sell toggle, price/amount inputs, advanced options
- **Order Book**: Bid/ask visualization with depth indicators
- **Price Chart**: Candlestick chart with time frame selection
- **Recent Trades**: Live trade feed with buy/sell indicators
- **Active Orders**: Order management with edit/cancel actions

## Getting Started

### Prerequisites

- Node.js 16+
- npm or yarn

### Installation

1. **Install dependencies**:

   ```bash
   npm install
   ```

2. **Start the development server**:

   ```bash
   npm start
   ```

3. **Build for production**:
   ```bash
   npm run build
   ```

### Environment Setup

The frontend connects to the Rust backend at `http://127.0.0.1:3000` by default. Make sure the backend server is running.

## Design System

### Colors

- **Background**: Dark gradient (`#0d1117` to `#161b22`)
- **Buy Actions**: Emerald green (`#10b981`)
- **Sell Actions**: Red (`#ef4444`)
- **Glass Effects**: Semi-transparent overlays with backdrop blur

### Typography

- **Primary Font**: Inter (clean, modern)
- **Monospace**: For prices and numbers
- **Weights**: 300-700 for hierarchy

### Animations

- **Page Load**: Fade-in and slide-up effects
- **Hover States**: Scale and glow effects
- **Data Updates**: Smooth number transitions
- **Button Interactions**: Scale feedback

## API Integration

The frontend communicates with the Rust backend through REST APIs:

- `POST /order` - Create new orders
- `DELETE /order` - Cancel orders
- `GET /order` - Fetch orderbook depth
- `GET /trades` - Get trade history

## Responsive Design

- **Desktop First**: Optimized for trading workstations
- **Grid Layout**: Flexible 3-panel layout
- **Mobile Ready**: Responsive breakpoints for smaller screens

## Performance

- **Optimized Rendering**: React.memo and useMemo for performance
- **Smooth Animations**: 60fps animations with Framer Motion
- **Efficient Updates**: Minimal re-renders with proper state management

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Contributing

1. Follow the existing code style
2. Use meaningful component and variable names
3. Add proper TypeScript types (when migrating)
4. Test animations on different devices
5. Ensure accessibility compliance

## License

MIT License - see LICENSE file for details
