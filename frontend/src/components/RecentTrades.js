import React from "react";
import { motion, AnimatePresence } from "framer-motion";
import { TrendingUp, TrendingDown } from "lucide-react";

const RecentTrades = ({ trades }) => {
  const TradeRow = ({ trade, index }) => (
    <motion.div
      key={trade.id}
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      transition={{ delay: index * 0.05 }}
      whileHover={{ backgroundColor: "rgba(255, 255, 255, 0.05)" }}
      className="table-row px-4 py-2 flex items-center justify-between text-sm"
    >
      <div className="flex items-center space-x-2">
        {trade.side === "buy" ? (
          <TrendingUp className="w-3 h-3 text-emerald-400" />
        ) : (
          <TrendingDown className="w-3 h-3 text-red-400" />
        )}
        <span
          className={`font-mono ${
            trade.side === "buy" ? "price-buy" : "price-sell"
          }`}
        >
          ${trade.price.toFixed(2)}
        </span>
      </div>
      <div className="text-gray-300 font-mono">{trade.quantity.toFixed(4)}</div>
      <div className="text-gray-400 text-xs">{trade.time}</div>
    </motion.div>
  );

  return (
    <div className="flex-1 glass border-b border-gray-800">
      {/* Header */}
      <div className="px-4 py-3 border-b border-gray-800">
        <h3 className="text-lg font-semibold text-white">Recent Trades</h3>
      </div>

      {/* Column Headers */}
      <div className="px-4 py-2 border-b border-gray-800">
        <div className="flex items-center justify-between text-xs text-gray-400 font-medium">
          <span>Price</span>
          <span>Amount</span>
          <span>Time</span>
        </div>
      </div>

      {/* Trades List */}
      <div className="h-48 overflow-y-auto scrollbar-hide">
        <AnimatePresence>
          {trades.map((trade, index) => (
            <TradeRow key={trade.id} trade={trade} index={index} />
          ))}
        </AnimatePresence>
      </div>

      {/* Live Indicator */}
      <div className="px-4 py-2 border-t border-gray-800">
        <div className="flex items-center space-x-2">
          <motion.div
            animate={{ opacity: [1, 0.3, 1] }}
            transition={{ duration: 2, repeat: Infinity }}
            className="w-2 h-2 bg-emerald-400 rounded-full"
          />
          <span className="text-xs text-gray-400">Live</span>
        </div>
      </div>
    </div>
  );
};

export default RecentTrades;
