# PowerShell test script for CEX orderbook system
Write-Host "Testing CEX Orderbook System" -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Green

# Start the server in background
Write-Host "Starting server..." -ForegroundColor Yellow
Start-Process -FilePath "cargo" -ArgumentList "run" -WindowStyle Hidden

# Wait for server to start
Start-Sleep -Seconds 3

# Test 1: Create a buy order
Write-Host "1. Creating buy order (User 1, Price: 100, Quantity: 10)" -ForegroundColor Cyan
$buyOrder = @{
    price = 100
    quantity = 10
    user_id = 1
    side = "Buy"
} | ConvertTo-Json

$response1 = Invoke-RestMethod -Uri "http://127.0.0.1:3000/order" -Method POST -Body $buyOrder -ContentType "application/json"
Write-Host "Response: $($response1 | ConvertTo-Json -Depth 3)" -ForegroundColor White

# Test 2: Create a sell order that should match
Write-Host "`n2. Creating sell order (User 2, Price: 95, Quantity: 5) - should match with buy order" -ForegroundColor Cyan
$sellOrder = @{
    price = 95
    quantity = 5
    user_id = 2
    side = "Sell"
} | ConvertTo-Json

$response2 = Invoke-RestMethod -Uri "http://127.0.0.1:3000/order" -Method POST -Body $sellOrder -ContentType "application/json"
Write-Host "Response: $($response2 | ConvertTo-Json -Depth 3)" -ForegroundColor White

# Test 3: Check orderbook depth
Write-Host "`n3. Getting orderbook depth" -ForegroundColor Cyan
$depth = Invoke-RestMethod -Uri "http://127.0.0.1:3000/order" -Method GET
Write-Host "Orderbook Depth: $($depth | ConvertTo-Json -Depth 3)" -ForegroundColor White

# Test 4: Get trade history
Write-Host "`n4. Getting trade history" -ForegroundColor Cyan
$trades = Invoke-RestMethod -Uri "http://127.0.0.1:3000/trades?limit=10" -Method GET
Write-Host "Trade History: $($trades | ConvertTo-Json -Depth 3)" -ForegroundColor White

Write-Host "`nTest completed!" -ForegroundColor Green
