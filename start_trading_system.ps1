# PowerShell script to start the complete CEX trading system
Write-Host "Starting CEX Trading System" -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Green

# Function to check if a port is in use
function Test-Port {
    param([int]$Port)
    try {
        $connection = New-Object System.Net.Sockets.TcpClient
        $connection.Connect("localhost", $Port)
        $connection.Close()
        return $true
    }
    catch {
        return $false
    }
}

# Check if backend port is available
if (Test-Port -Port 3000) {
    Write-Host "Port 3000 is already in use. Please stop the existing service first." -ForegroundColor Red
    exit 1
}

# Check if frontend port is available
if (Test-Port -Port 3001) {
    Write-Host "Port 3001 is already in use. Please stop the existing service first." -ForegroundColor Red
    exit 1
}

Write-Host "Starting Rust backend server..." -ForegroundColor Yellow

# Start the Rust backend
$backendJob = Start-Job -ScriptBlock {
    Set-Location "D:\Superteam\cex"
    cargo run
}

# Wait a moment for backend to start
Start-Sleep -Seconds 3

# Check if backend started successfully
if (Test-Port -Port 3000) {
    Write-Host "✓ Backend server started successfully on port 3000" -ForegroundColor Green
} else {
    Write-Host "✗ Failed to start backend server" -ForegroundColor Red
    Stop-Job $backendJob
    Remove-Job $backendJob
    exit 1
}

Write-Host "Starting React frontend..." -ForegroundColor Yellow

# Start the React frontend
$frontendJob = Start-Job -ScriptBlock {
    Set-Location "D:\Superteam\cex\frontend"
    npm start
}

# Wait for frontend to start
Start-Sleep -Seconds 5

Write-Host "✓ Frontend development server starting on port 3001" -ForegroundColor Green
Write-Host ""
Write-Host "Trading System Status:" -ForegroundColor Cyan
Write-Host "- Backend API: http://127.0.0.1:3000" -ForegroundColor White
Write-Host "- Frontend UI: http://127.0.0.1:3001" -ForegroundColor White
Write-Host ""
Write-Host "Press Ctrl+C to stop both services" -ForegroundColor Yellow

# Keep the script running and show job status
try {
    while ($true) {
        Start-Sleep -Seconds 5
        
        # Check if backend is still running
        if (-not (Test-Port -Port 3000)) {
            Write-Host "Backend server stopped unexpectedly" -ForegroundColor Red
            break
        }
        
        # Check if frontend is still running
        if (-not (Test-Port -Port 3001)) {
            Write-Host "Frontend server stopped unexpectedly" -ForegroundColor Red
            break
        }
    }
}
finally {
    Write-Host ""
    Write-Host "Stopping services..." -ForegroundColor Yellow
    
    # Stop both jobs
    Stop-Job $backendJob, $frontendJob -ErrorAction SilentlyContinue
    Remove-Job $backendJob, $frontendJob -ErrorAction SilentlyContinue
    
    Write-Host "✓ All services stopped" -ForegroundColor Green
}
