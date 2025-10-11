import React from "react";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { Line } from "react-chartjs-2";
import { motion } from "framer-motion";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

const PriceChart = () => {
  // Mock candlestick data
  const data = {
    labels: ["09:00", "10:00", "11:00", "12:00", "13:00", "14:00", "15:00"],
    datasets: [
      {
        label: "ETH/USDC",
        data: [3200, 3220, 3180, 3250, 3240, 3245, 3250],
        borderColor: "#10b981",
        backgroundColor: "rgba(16, 185, 129, 0.1)",
        borderWidth: 2,
        fill: true,
        tension: 0.4,
        pointBackgroundColor: "#10b981",
        pointBorderColor: "#ffffff",
        pointBorderWidth: 2,
        pointRadius: 4,
        pointHoverRadius: 6,
      },
    ],
  };

  const options = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      tooltip: {
        backgroundColor: "rgba(0, 0, 0, 0.8)",
        titleColor: "#ffffff",
        bodyColor: "#ffffff",
        borderColor: "#374151",
        borderWidth: 1,
        cornerRadius: 8,
        displayColors: false,
        callbacks: {
          label: function (context) {
            return `Price: $${context.parsed.y.toFixed(2)}`;
          },
        },
      },
    },
    scales: {
      x: {
        display: true,
        grid: {
          color: "rgba(55, 65, 81, 0.3)",
          drawBorder: false,
        },
        ticks: {
          color: "#9ca3af",
          font: {
            size: 12,
          },
        },
      },
      y: {
        display: true,
        grid: {
          color: "rgba(55, 65, 81, 0.3)",
          drawBorder: false,
        },
        ticks: {
          color: "#9ca3af",
          font: {
            size: 12,
          },
          callback: function (value) {
            return "$" + value.toFixed(0);
          },
        },
      },
    },
    interaction: {
      intersect: false,
      mode: "index",
    },
    elements: {
      point: {
        hoverBackgroundColor: "#ffffff",
      },
    },
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="h-full glass rounded-xl p-6"
    >
      {/* Chart Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h3 className="text-xl font-bold text-white">ETH/USDC</h3>
          <div className="flex items-center space-x-4 mt-1">
            <span className="text-2xl font-bold text-white">$3,245.75</span>
            <span className="text-sm text-emerald-400 bg-emerald-400/10 px-2 py-1 rounded">
              +2.45%
            </span>
          </div>
        </div>
        <div className="flex space-x-2">
          {["1m", "5m", "15m", "1h", "4h", "1d"].map((timeframe) => (
            <motion.button
              key={timeframe}
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              className="px-3 py-1 text-xs font-medium text-gray-400 hover:text-white hover:bg-gray-800 rounded transition-colors"
            >
              {timeframe}
            </motion.button>
          ))}
        </div>
      </div>

      {/* Chart */}
      <div className="h-80">
        <Line data={data} options={options} />
      </div>

      {/* Chart Stats */}
      <div className="grid grid-cols-4 gap-4 mt-6">
        <div className="text-center">
          <div className="text-sm text-gray-400">24h High</div>
          <div className="text-lg font-semibold text-white">$3,280.50</div>
        </div>
        <div className="text-center">
          <div className="text-sm text-gray-400">24h Low</div>
          <div className="text-lg font-semibold text-white">$3,180.25</div>
        </div>
        <div className="text-center">
          <div className="text-sm text-gray-400">24h Volume</div>
          <div className="text-lg font-semibold text-white">1,245.6 ETH</div>
        </div>
        <div className="text-center">
          <div className="text-sm text-gray-400">Market Cap</div>
          <div className="text-lg font-semibold text-white">$390.2B</div>
        </div>
      </div>
    </motion.div>
  );
};

export default PriceChart;
