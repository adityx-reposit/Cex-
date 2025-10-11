import React from "react";
import { motion } from "framer-motion";
import { Edit3, X, Clock, CheckCircle, AlertCircle } from "lucide-react";

const ActiveOrders = ({ orders }) => {
  const getStatusIcon = (status) => {
    switch (status) {
      case "active":
        return <Clock className="w-3 h-3 text-blue-400" />;
      case "partial":
        return <AlertCircle className="w-3 h-3 text-yellow-400" />;
      case "filled":
        return <CheckCircle className="w-3 h-3 text-emerald-400" />;
      default:
        return <Clock className="w-3 h-3 text-gray-400" />;
    }
  };

  const getStatusColor = (status) => {
    switch (status) {
      case "active":
        return "text-blue-400";
      case "partial":
        return "text-yellow-400";
      case "filled":
        return "text-emerald-400";
      default:
        return "text-gray-400";
    }
  };

  const OrderRow = ({ order, index }) => (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.1 }}
      whileHover={{ backgroundColor: "rgba(255, 255, 255, 0.05)" }}
      className="table-row px-4 py-3"
    >
      <div className="space-y-2">
        {/* Order Header */}
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <div
              className={`w-2 h-2 rounded-full ${
                order.side === "buy" ? "bg-emerald-400" : "bg-red-400"
              }`}
            />
            <span
              className={`text-sm font-medium uppercase ${
                order.side === "buy" ? "text-emerald-400" : "text-red-400"
              }`}
            >
              {order.side}
            </span>
          </div>
          <div className="flex items-center space-x-1">
            {getStatusIcon(order.status)}
            <span
              className={`text-xs font-medium ${getStatusColor(order.status)}`}
            >
              {order.status}
            </span>
          </div>
        </div>

        {/* Order Details */}
        <div className="grid grid-cols-2 gap-2 text-xs">
          <div>
            <span className="text-gray-400">Price:</span>
            <span className="text-white font-mono ml-1">
              ${order.price.toFixed(2)}
            </span>
          </div>
          <div>
            <span className="text-gray-400">Amount:</span>
            <span className="text-white font-mono ml-1">
              {order.quantity.toFixed(4)}
            </span>
          </div>
        </div>

        {/* Progress Bar */}
        <div className="space-y-1">
          <div className="flex justify-between text-xs">
            <span className="text-gray-400">Filled</span>
            <span className="text-white">
              {((order.filled / order.quantity) * 100).toFixed(1)}%
            </span>
          </div>
          <div className="w-full bg-gray-700 rounded-full h-1">
            <motion.div
              initial={{ width: 0 }}
              animate={{ width: `${(order.filled / order.quantity) * 100}%` }}
              transition={{ duration: 0.8, delay: index * 0.1 }}
              className={`h-1 rounded-full ${
                order.side === "buy" ? "bg-emerald-400" : "bg-red-400"
              }`}
            />
          </div>
        </div>

        {/* Time and Actions */}
        <div className="flex items-center justify-between">
          <span className="text-xs text-gray-400">{order.time}</span>
          <div className="flex items-center space-x-2">
            <motion.button
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.9 }}
              className="p-1 text-gray-400 hover:text-blue-400 transition-colors"
              title="Edit Order"
            >
              <Edit3 className="w-3 h-3" />
            </motion.button>
            <motion.button
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.9 }}
              className="p-1 text-gray-400 hover:text-red-400 transition-colors"
              title="Cancel Order"
            >
              <X className="w-3 h-3" />
            </motion.button>
          </div>
        </div>
      </div>
    </motion.div>
  );

  return (
    <div className="flex-1 glass">
      {/* Header */}
      <div className="px-4 py-3 border-b border-gray-800">
        <h3 className="text-lg font-semibold text-white">Active Orders</h3>
      </div>

      {/* Orders List */}
      <div className="h-64 overflow-y-auto scrollbar-hide">
        {orders.length > 0 ? (
          orders.map((order, index) => (
            <OrderRow key={order.id} order={order} index={index} />
          ))
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="w-12 h-12 mx-auto mb-3 rounded-full bg-gray-800 flex items-center justify-center">
                <Clock className="w-6 h-6 text-gray-500" />
              </div>
              <p className="text-gray-400 text-sm">No active orders</p>
              <p className="text-gray-500 text-xs mt-1">
                Place an order to see it here
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default ActiveOrders;
