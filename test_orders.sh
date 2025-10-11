#!/bin/bash

# Test script for CEX orderbook system
echo "Testing CEX Orderbook System"
echo "============================="

# Test 1: Create a buy order
echo "1. Creating buy order (User 1, Price: 100, Quantity: 10)"
curl -X POST http://127.0.0.1:3000/order \
  -H "Content-Type: application/json" \
  -d '{"price": 100, "quantity": 10, "user_id": 1, "side": "Buy"}' \
  | jq '.'

echo -e "\n"

# Test 2: Create a sell order that should match
echo "2. Creating sell order (User 2, Price: 95, Quantity: 5) - should match with buy order"
curl -X POST http://127.0.0.1:3000/order \
  -H "Content-Type: application/json" \
  -d '{"price": 95, "quantity": 5, "user_id": 2, "side": "Sell"}' \
  | jq '.'

echo -e "\n"

# Test 3: Create another sell order
echo "3. Creating another sell order (User 3, Price: 105, Quantity: 8)"
curl -X POST http://127.0.0.1:3000/order \
  -H "Content-Type: application/json" \
  -d '{"price": 105, "quantity": 8, "user_id": 3, "side": "Sell"}' \
  | jq '.'

echo -e "\n"

# Test 4: Check orderbook depth
echo "4. Getting orderbook depth"
curl -X GET http://127.0.0.1:3000/order \
  | jq '.'

echo -e "\n"

# Test 5: Get trade history
echo "5. Getting trade history"
curl -X GET http://127.0.0.1:3000/trades?limit=10 \
  | jq '.'

echo -e "\n"

# Test 6: Create another buy order that should match with remaining sell orders
echo "6. Creating buy order (User 4, Price: 110, Quantity: 15) - should match remaining sell orders"
curl -X POST http://127.0.0.1:3000/order \
  -H "Content-Type: application/json" \
  -d '{"price": 110, "quantity": 15, "user_id": 4, "side": "Buy"}' \
  | jq '.'

echo -e "\n"

# Test 7: Final orderbook state
echo "7. Final orderbook depth"
curl -X GET http://127.0.0.1:3000/order \
  | jq '.'

echo -e "\n"

# Test 8: Final trade history
echo "8. Final trade history"
curl -X GET http://127.0.0.1:3000/trades?limit=20 \
  | jq '.'

echo -e "\nTest completed!"
