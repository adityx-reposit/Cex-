import React, { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  TrendingUp,
  TrendingDown,
  Wallet,
  Settings,
  ChevronDown,
} from "lucide-react";

const OrderForm = () => {
  const [orderType, setOrderType] = useState("buy");
  const [orderMode, setOrderMode] = useState("limit");
  const [price, setPrice] = useState("3245.75");
  const [amount, setAmount] = useState("");
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [liquiditySource, setLiquiditySource] = useState("spot");

  const handleSubmit = (e) => {
    e.preventDefault();
    // Handle order submission
    console.log("Order submitted:", { orderType, orderMode, price, amount });
  };

  const setMaxAmount = () => {
    // Set maximum available balance
    setAmount("1.25");
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="space-y-6"
    >
      {/* Token Pair Selector */}
      <div className="glass rounded-xl p-4">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-white">ETH/USDC</h3>
          <div className="flex items-center space-x-2">
            <div className="w-6 h-6 bg-blue-500 rounded-full"></div>
            <ChevronDown className="w-4 h-4 text-gray-400" />
          </div>
        </div>
        <div className="text-sm text-gray-400">Last: $3,245.75</div>
      </div>

      {/* Buy/Sell Toggle */}
      <div className="glass rounded-xl p-4">
        <div className="flex rounded-lg bg-gray-800 p-1">
          <motion.button
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            onClick={() => setOrderType("buy")}
            className={`flex-1 flex items-center justify-center space-x-2 py-3 rounded-md font-medium transition-all duration-300 ${
              orderType === "buy"
                ? "bg-emerald-600 text-white shadow-lg shadow-emerald-500/25"
                : "text-gray-400 hover:text-white"
            }`}
          >
            <TrendingUp className="w-4 h-4" />
            <span>Buy</span>
          </motion.button>
          <motion.button
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            onClick={() => setOrderType("sell")}
            className={`flex-1 flex items-center justify-center space-x-2 py-3 rounded-md font-medium transition-all duration-300 ${
              orderType === "sell"
                ? "bg-red-600 text-white shadow-lg shadow-red-500/25"
                : "text-gray-400 hover:text-white"
            }`}
          >
            <TrendingDown className="w-4 h-4" />
            <span>Sell</span>
          </motion.button>
        </div>
      </div>

      {/* Order Form */}
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Price Input */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Price (USDC)
          </label>
          <input
            type="number"
            value={price}
            onChange={(e) => setPrice(e.target.value)}
            className="input-field"
            placeholder="0.00"
            step="0.01"
          />
        </div>

        {/* Amount Input */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <label className="block text-sm font-medium text-gray-300">
              Amount (ETH)
            </label>
            <button
              type="button"
              onClick={setMaxAmount}
              className="text-xs text-emerald-400 hover:text-emerald-300 transition-colors"
            >
              Max
            </button>
          </div>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="input-field"
            placeholder="0.00"
            step="0.001"
          />
        </div>

        {/* Total */}
        <div className="glass rounded-lg p-3">
          <div className="flex justify-between text-sm">
            <span className="text-gray-400">Total</span>
            <span className="text-white font-medium">
              ${(parseFloat(price) * parseFloat(amount || 0)).toFixed(2)}
            </span>
          </div>
        </div>

        {/* Advanced Options */}
        <div>
          <motion.button
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            type="button"
            onClick={() => setShowAdvanced(!showAdvanced)}
            className="flex items-center justify-between w-full p-3 glass rounded-lg hover:bg-gray-800/30 transition-colors"
          >
            <div className="flex items-center space-x-2">
              <Settings className="w-4 h-4 text-gray-400" />
              <span className="text-sm font-medium text-gray-300">
                Advanced
              </span>
            </div>
            <motion.div
              animate={{ rotate: showAdvanced ? 180 : 0 }}
              transition={{ duration: 0.2 }}
            >
              <ChevronDown className="w-4 h-4 text-gray-400" />
            </motion.div>
          </motion.button>

          <AnimatePresence>
            {showAdvanced && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: "auto" }}
                exit={{ opacity: 0, height: 0 }}
                transition={{ duration: 0.3 }}
                className="mt-3 space-y-3 overflow-hidden"
              >
                {/* Order Type */}
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Order Type
                  </label>
                  <select
                    value={orderMode}
                    onChange={(e) => setOrderMode(e.target.value)}
                    className="input-field"
                  >
                    <option value="limit">Limit Order</option>
                    <option value="market">Market Order</option>
                  </select>
                </div>

                {/* Liquidity Source */}
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Liquidity Source
                  </label>
                  <select
                    value={liquiditySource}
                    onChange={(e) => setLiquiditySource(e.target.value)}
                    className="input-field"
                  >
                    <option value="spot">Spot</option>
                    <option value="margin">Margin</option>
                  </select>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Wallet Balance */}
        <div className="glass rounded-lg p-3">
          <div className="flex items-center space-x-2 mb-2">
            <Wallet className="w-4 h-4 text-gray-400" />
            <span className="text-sm font-medium text-gray-300">
              Wallet Balance
            </span>
          </div>
          <div className="space-y-1">
            <div className="flex justify-between text-sm">
              <span className="text-gray-400">ETH</span>
              <span className="text-white">1.25</span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-400">USDC</span>
              <span className="text-white">4,056.88</span>
            </div>
          </div>
        </div>

        {/* Submit Button */}
        <motion.button
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
          type="submit"
          className={`w-full py-4 rounded-xl font-semibold text-lg transition-all duration-300 ${
            orderType === "buy"
              ? "btn-buy hover:shadow-emerald-500/30"
              : "btn-sell hover:shadow-red-500/30"
          }`}
        >
          {orderType === "buy" ? "Buy ETH" : "Sell ETH"}
        </motion.button>
      </form>
    </motion.div>
  );
};

export default OrderForm;
