# axum-todo-app


<!-- cargo fix --bin "axum-todo-app" -->

<!-- 


  1. Using cURL (Command Line)
Test Root Endpoint
bash
curl http://localhost:9000
Register a User
bash
curl -X POST http://localhost:9000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "password123"}'
Login
bash
curl -X POST http://localhost:9000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "password123"}'
This should return a JWT token. Copy it for the next requests.
Create a Todo (with JWT)
bash
curl -X POST http://localhost:9000/api/todos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE" \
  -d '{"title": "Buy groceries"}'
Get All Todos
bashcurl http://localhost:9000/api/todos \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE"
Update a Todo
bashcurl -X PUT http://localhost:9000/api/todos/1 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE" \
  -d '{"title": "Buy groceries and milk", "completed": true}'
Delete a Todo
bashcurl -X DELETE http://localhost:9000/api/todos/1 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE"
2. Using HTTPie (More User-Friendly)
Install: brew install httpie or pip install httpie
bash# Register
http POST :9000/api/auth/register username=alice password=password123

# Login
http POST :9000/api/auth/login username=alice password=password123

# Create todo (replace TOKEN)
http POST :9000/api/todos title="Buy milk" Authorization:"Bearer TOKEN"

# Get todos
http GET :9000/api/todos Authorization:"Bearer TOKEN"
3. Using Postman or Insomnia (GUI)

Download Postman or Insomnia
Create requests with nice UI
Save collections for reuse
Easier to manage tokens

4. Using Thunder Client (VS Code Extension)

Install "Thunder Client" in VS Code
Create requests directly in your editor
Very convenient for development

5. Using a Test Script
Create test.sh:
bash#!/bin/bash

BASE_URL="http://localhost:9000"

echo "=== Registering user ==="
curl -X POST $BASE_URL/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "test123"}'

echo -e "\n\n=== Logging in ==="
TOKEN=$(curl -s -X POST $BASE_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "password": "test123"}' \
  | grep -o '"token":"[^"]*' | cut -d'"' -f4)

echo "Token: $TOKEN"

echo -e "\n\n=== Creating todo ==="
curl -X POST $BASE_URL/api/todos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"title": "Test todo"}'

echo -e "\n\n=== Getting todos ==="
curl $BASE_URL/api/todos \
  -H "Authorization: Bearer $TOKEN"
Run it:
bashchmod +x test.sh
./test.sh
Quick Test Flow
bash# 1. Register
curl -X POST http://localhost:9000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "pass123"}'

# 2. Login and save token
TOKEN=$(curl -s -X POST http://localhost:9000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "pass123"}' \
  | jq -r '.token')

echo $TOKEN

# 3. Create todo
curl -X POST http://localhost:9000/api/todos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"title": "Learn Rust"}'

# 4. Get todos
curl http://localhost:9000/api/todos \
  -H "Authorization: Bearer $TOKEN"
Note: The exact endpoints depend on what routes you've defined in your web module. Adjust the URLs based on your actual route definitions! -->