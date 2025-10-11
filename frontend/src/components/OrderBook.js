import React from "react";
import { motion } from "framer-motion";

const OrderBook = ({ data }) => {
  const { bids, asks } = data;

  const OrderRow = ({ order, isBid, index }) => (
    <motion.div
      initial={{ opacity: 0, x: isBid ? -20 : 20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: index * 0.05 }}
      whileHover={{ backgroundColor: "rgba(255, 255, 255, 0.05)" }}
      className="table-row px-4 py-2 flex items-center justify-between text-sm relative group"
    >
      <div className="flex-1 flex items-center justify-between">
        <span className={`font-mono ${isBid ? "price-buy" : "price-sell"}`}>
          ${order.price.toFixed(2)}
        </span>
        <span className="text-gray-300 font-mono">
          {order.quantity.toFixed(4)}
        </span>
        <span className="text-gray-400 font-mono">
          {order.total.toFixed(2)}
        </span>
      </div>

      {/* Depth Bar */}
      <div className="absolute inset-y-0 left-0 opacity-20">
        <motion.div
          className={`h-full ${isBid ? "bg-emerald-500" : "bg-red-500"}`}
          initial={{ width: 0 }}
          animate={{
            width: `${
              (order.quantity /
                Math.max(
                  ...bids.map((b) => b.quantity),
                  ...asks.map((a) => a.quantity)
                )) *
              100
            }%`,
          }}
          transition={{ duration: 0.5, delay: index * 0.1 }}
        />
      </div>
    </motion.div>
  );

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="glass border-b border-gray-800 px-4 py-3">
        <h3 className="text-lg font-semibold text-white mb-3">Order Book</h3>
        <div className="flex items-center justify-between text-xs text-gray-400 font-medium">
          <span>Price (USDC)</span>
          <span>Amount (ETH)</span>
          <span>Total (USDC)</span>
        </div>
      </div>

      {/* Order Book Content */}
      <div className="flex-1 flex">
        {/* Asks (Sell Orders) */}
        <div className="flex-1">
          <div className="px-2 py-1 bg-red-500/10 border-b border-red-500/20">
            <span className="text-xs font-medium text-red-400">Asks</span>
          </div>
          <div className="h-full overflow-y-auto scrollbar-hide">
            {asks
              .slice()
              .reverse()
              .map((ask, index) => (
                <OrderRow key={index} order={ask} isBid={false} index={index} />
              ))}
          </div>
        </div>

        {/* Spread */}
        <div className="w-px bg-gray-800"></div>

        {/* Bids (Buy Orders) */}
        <div className="flex-1">
          <div className="px-2 py-1 bg-emerald-500/10 border-b border-emerald-500/20">
            <span className="text-xs font-medium text-emerald-400">Bids</span>
          </div>
          <div className="h-full overflow-y-auto scrollbar-hide">
            {bids.map((bid, index) => (
              <OrderRow key={index} order={bid} isBid={true} index={index} />
            ))}
          </div>
        </div>
      </div>

      {/* Spread Info */}
      <div className="glass border-t border-gray-800 px-4 py-2">
        <div className="flex justify-between text-xs text-gray-400">
          <span>Spread</span>
          <span className="text-white">
            {asks.length > 0 && bids.length > 0
              ? `${(asks[0].price - bids[0].price).toFixed(2)} USDC`
              : "N/A"}
          </span>
        </div>
      </div>
    </div>
  );
};

export default OrderBook;
