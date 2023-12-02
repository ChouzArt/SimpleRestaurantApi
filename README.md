# Restaurant Order Management System

This repository contains a Rust-based web application for managing restaurant orders, designed to simulate a real-world business application using functional programming principles as part of an interview [coding assignment](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md).

## Features

- Add, remove, and query orders using a RESTful API.
- Simultaneous handling of at least 10 incoming add/remove/query requests.
- Integration tests simulating multiple clients.
- Use of a PostgreSQL database for persistent storage.
- Containerized application deployment using Docker.

## Prerequisites

- Rust (cargo)
- Docker and Docker Compose

## Setup

Before you begin, ensure Docker and Docker Compose are installed on your system and accessible via the command line.

## How to Run

Clone the repository to your local machine:

```
git clone https://github.com/your-github-username/restaurant-system.git
```

Use Docker Compose to build and run the services. The command will set up the PostgreSQL database, run database migrations, and start the application server on the specified port (default: 8080).
```
docker compose -f "docker-compose.yml" up -d --build
```

## How to Test

To execute the integration tests, run:
```
cargo test
```

This will run all the tests in the project, including client simulation tests which will produce test traffic to the API.

## APIs

Below are some example API calls that can be performed after starting the application:

**Add Order:**
```
POST /v1/orders Content-Type: application/json

{ "table_number": 1, "menu_item_id": 10 }
```

**Get All Orders from a Table:**
```
GET /v1/tables/{table_number}/orders
```

**Remove Order by Order ID:**
```
DELETE /v1/orders/{order_id}
```

**Remove Order by Menu Item ID (Latest order):**
```
DELETE /v1/tables/{table_number}/menu_items/{menu_item_id}
```

Replace `{table_number}`, `{menu_item_id}`, and `{order_id}` with actual values.

## Expected Outputs

CREATE - The add order API will return the UUID of the newly created order. 

READ - Query APIs will return a list or a single order in JSON format. 

DELETE - The delete API will confirm deletion with amount of affected rows.

## Example Calls

Here are some example calls with `curl`, assuming the server is running locally on port 8080:

Add order
```bash
curl -X POST http://localhost:8080/v1/orders -H "Content-Type: application/json" -d '{"table_number": 1, "menu_item_id": 5}'
```

Get order
Get All Orders for Table 1:
```bash
curl http://localhost:8080/v1/tables/1/orders
```

Remove an Order by ID:
```bash
curl -X DELETE http://localhost:8080/v1/orders/{order_id}
```
Replace {order_id} with the ID of the order you want to delete.

## MIT License

Copyright (c) 2023 Carlos Chouza

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


