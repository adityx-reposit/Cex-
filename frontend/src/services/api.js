import axios from "axios";

const API_BASE_URL = "http://127.0.0.1:3000";

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request interceptor for logging
api.interceptors.request.use(
  (config) => {
    console.log(
      `Making ${config.method?.toUpperCase()} request to ${config.url}`
    );
    return config;
  },
  (error) => {
    console.error("Request error:", error);
    return Promise.reject(error);
  }
);

// Response interceptor for error handling
api.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    console.error("API Error:", error.response?.data || error.message);
    return Promise.reject(error);
  }
);

export const tradingAPI = {
  // Create a new order
  createOrder: async (orderData) => {
    try {
      const response = await api.post("/order", orderData);
      return response.data;
    } catch (error) {
      throw new Error(
        error.response?.data?.message || "Failed to create order"
      );
    }
  },

  // Cancel an order
  cancelOrder: async (orderId) => {
    try {
      const response = await api.delete("/order", {
        data: { order_id: orderId },
      });
      return response.data;
    } catch (error) {
      throw new Error(
        error.response?.data?.message || "Failed to cancel order"
      );
    }
  },

  // Get orderbook depth
  getOrderbookDepth: async () => {
    try {
      const response = await api.get("/order");
      return response.data;
    } catch (error) {
      throw new Error(
        error.response?.data?.message || "Failed to fetch orderbook"
      );
    }
  },

  // Get trade history
  getTradeHistory: async (limit = 100) => {
    try {
      const response = await api.get(`/trades?limit=${limit}`);
      return response.data;
    } catch (error) {
      throw new Error(
        error.response?.data?.message || "Failed to fetch trades"
      );
    }
  },
};

export default api;
