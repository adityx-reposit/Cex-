import React, { useState, useEffect } from "react";
import { motion } from "framer-motion";
import OrderForm from "./OrderForm";
import OrderBook from "./OrderBook";
import PriceChart from "./PriceChart";
import ActiveOrders from "./ActiveOrders";
import RecentTrades from "./RecentTrades";
import Tabs from "./Tabs";
import { TrendingUp, BarChart3, Activity } from "lucide-react";

const TradingInterface = () => {
  const [activeTab, setActiveTab] = useState("price-chart");
  const [orderbookData, setOrderbookData] = useState({
    bids: [],
    asks: [],
  });
  const [trades, setTrades] = useState([]);
  const [activeOrders, setActiveOrders] = useState([]);

  // Mock data - replace with actual API calls
  useEffect(() => {
    // Simulate initial data loading
    setOrderbookData({
      bids: [
        { price: 3245.5, quantity: 1.25, total: 4056.88 },
        { price: 3245.25, quantity: 2.1, total: 6815.03 },
        { price: 3245.0, quantity: 0.85, total: 2758.25 },
        { price: 3244.75, quantity: 3.4, total: 11032.15 },
        { price: 3244.5, quantity: 1.6, total: 5191.2 },
      ],
      asks: [
        { price: 3246.0, quantity: 1.8, total: 5842.8 },
        { price: 3246.25, quantity: 2.3, total: 7466.38 },
        { price: 3246.5, quantity: 0.95, total: 3084.18 },
        { price: 3246.75, quantity: 1.45, total: 4707.79 },
        { price: 3247.0, quantity: 2.2, total: 7143.4 },
      ],
    });

    setTrades([
      { id: 1, price: 3245.75, quantity: 0.5, side: "buy", time: "14:23:45" },
      { id: 2, price: 3245.5, quantity: 1.2, side: "sell", time: "14:23:42" },
      { id: 3, price: 3246.0, quantity: 0.8, side: "buy", time: "14:23:38" },
      { id: 4, price: 3245.25, quantity: 2.1, side: "sell", time: "14:23:35" },
      { id: 5, price: 3245.75, quantity: 0.3, side: "buy", time: "14:23:31" },
    ]);

    setActiveOrders([
      {
        id: 1,
        side: "buy",
        price: 3244.0,
        quantity: 1.5,
        filled: 0,
        status: "active",
        time: "14:20:15",
      },
      {
        id: 2,
        side: "sell",
        price: 3247.5,
        quantity: 2.0,
        filled: 0.8,
        status: "partial",
        time: "14:18:32",
      },
      {
        id: 3,
        side: "buy",
        price: 3243.75,
        quantity: 0.9,
        filled: 0,
        status: "active",
        time: "14:15:44",
      },
    ]);
  }, []);

  const tabs = [
    { id: "price-chart", label: "Price Chart", icon: TrendingUp },
    { id: "depth-chart", label: "Depth Chart", icon: BarChart3 },
    { id: "order-lines", label: "Order Lines", icon: Activity },
  ];

  return (
    <div className="min-h-screen bg-dark-gradient">
      {/* Header */}
      <motion.header
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        className="glass border-b border-gray-800 px-6 py-4"
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <div className="w-8 h-8 bg-gradient-to-r from-emerald-500 to-blue-500 rounded-lg flex items-center justify-center">
              <TrendingUp className="w-5 h-5 text-white" />
            </div>
            <h1 className="text-xl font-bold text-white">CEX Trading</h1>
          </div>
          <div className="flex items-center space-x-4">
            <div className="text-sm text-gray-400">ETH/USDC</div>
            <div className="text-lg font-bold text-white">$3,245.75</div>
            <div className="text-sm text-emerald-400">+2.45%</div>
          </div>
        </div>
      </motion.header>

      {/* Main Trading Interface */}
      <div className="flex h-screen">
        {/* Left Panel - Order Form */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.1 }}
          className="w-80 glass border-r border-gray-800 p-6"
        >
          <OrderForm />
        </motion.div>

        {/* Center Panel - Charts and Order Book */}
        <div className="flex-1 flex flex-col">
          {/* Tabs */}
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="glass border-b border-gray-800"
          >
            <Tabs
              tabs={tabs}
              activeTab={activeTab}
              onTabChange={setActiveTab}
            />
          </motion.div>

          {/* Chart Area */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="flex-1 p-6"
          >
            {activeTab === "price-chart" && <PriceChart />}
            {activeTab === "depth-chart" && (
              <div className="text-center text-gray-400">
                Depth Chart Coming Soon
              </div>
            )}
            {activeTab === "order-lines" && (
              <div className="text-center text-gray-400">
                Order Lines Coming Soon
              </div>
            )}
          </motion.div>

          {/* Order Book */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
            className="h-80 glass border-t border-gray-800"
          >
            <OrderBook data={orderbookData} />
          </motion.div>
        </div>

        {/* Right Panel - Trades and Orders */}
        <motion.div
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.5 }}
          className="w-80 glass border-l border-gray-800 flex flex-col"
        >
          <RecentTrades trades={trades} />
          <ActiveOrders orders={activeOrders} />
        </motion.div>
      </div>
    </div>
  );
};

export default TradingInterface;
