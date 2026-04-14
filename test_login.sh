#!/bin/bash
# Test the login functionality

# Test 1: Check if backend is responding
echo "Testing backend health..."
curl -s http://localhost:3000/health && echo "✓ Backend is healthy" || echo "✗ Backend is not responding"

# Test 2: Try to login with admin credentials
echo ""
echo "Testing login with admin/Admin@123..."
curl -s -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "request": {
      "username": "admin",
      "password": "Admin@123"
    }
  }' | python -m json.tool 2>/dev/null || echo "Login attempt completed"

echo ""
echo "If you see a token in the response above, login is working!"
