# BetStream 🚀

[![Rust](https://img.shields.io/badge/Rust-1.72+-orange?logo=rust)](https://www.rust-lang.org/) 
[![React](https://img.shields.io/badge/React-18-blue?logo=react)](https://reactjs.org/)
[![Swagger](https://img.shields.io/badge/Swagger-OpenAPI_3.0-green?logo=swagger)](https://swagger.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

**BetStream** is a real-time betting operations platform for managing, processing, and monitoring bets through structured workflows. It is designed for operators and automated systems that require live updates and control over account-based bet batches.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Core Concepts](#core-concepts)
  - [Accounts](#accounts)
  - [Batches](#batches)
  - [Bets](#bets)
- [Real-time Events (SSE)](#real-time-events-sse)
- [Web Interface](#web-interface)
- [API Documentation](#api-documentation)
- [Local Development](#local-development)
- [Docker Setup](#docker-setup)
- [Database Seeding](#database-seeding)
- [API Reference](#api-reference)
- [Production Notes](#production-notes)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

BetStream acts as a **bet execution control plane**. Operators interact with a web interface to:

- View accounts
- Manage batches of bets
- Update bet statuses (pending, successful, failed)
- Submit batches for execution
- Monitor real-time updates

The backend stores all accounts, batches, and bets, emits events on changes, and supports integration with betting vendors.

---

## Features

- ✅ Account-based bet management
- ✅ Batch processing workflow
- ✅ Real-time updates via Server-Sent Events (SSE)
- ✅ Interactive operator web UI
- ✅ Manual and programmatic bet updates
- ✅ **Interactive API Documentation (Swagger UI)**
- ✅ **OpenAPI 3.0 Specification**
- ✅ Persistent storage with SQLite (or extendable to Postgres)
- ✅ Dockerized development and production environment

---

## Architecture
```
+-----------------+   REST / SSE   +------------------------+
|                 | <------------> |                        |
|   React UI      |                | BetStream API (Rust)   |
|  (Frontend)     |                |   + Axum / SQLx        |
|                 |                |   + Swagger UI         |
+-----------------+                +------------------------+
                                            |
                                            v
                                   +-------------------+
                                   | SQLite / Postgres |
                                   | Accounts, Batches |
                                   |      Bets         |
                                   +-------------------+

[Nginx] → [BetStream API + Frontend + Swagger]
```

---

## Core Concepts

### Accounts
An account represents a betting entity (bookmaker or trading account). Each account can own multiple batches of bets.

### Batches
A batch is a group of bets that are logically processed together.

Each batch contains:

- Metadata (name, strategy, parameters)
- A list of bets
- Status flags (active or completed)

Lifecycle:
```
Created → Active → Submitted → Completed
```

### Bets
Each bet contains:

- Selection details
- Stake and cost
- Status (pending, successful, failed)

Bets can be updated manually via the UI or programmatically.

---

## Real-time Events (SSE)

BetStream emits events to connected clients via SSE:

- `account_created`
- `account_updated`
- `account_deleted`
- `batch_created`
- `batch_completed`
- `bet_status_updated`
- `batch_bets_updated`
- `keep-alive` (ping)

This allows:

- Live dashboards
- Multi-operator synchronization
- Real-time monitoring and automation

---

## Web Interface

The React frontend provides:

- Sidebar of accounts
- Batch selector per account
- Batch metadata display
- Bet tables with status controls
- Real-time updates via SSE
- Automatic selection of next batch when current batch is submitted/completed

---

## API Documentation

BetStream includes **interactive API documentation** powered by Swagger UI.

### Accessing Swagger UI

**Local Development:**
```
http://localhost:3001/swagger-ui
```

**Docker:**
```
http://localhost:3001/swagger-ui
```

**Production (via Nginx):**
```
https://your-domain.com/swagger-ui
```

### OpenAPI Specification

You can also access the raw OpenAPI JSON specification:
```
http://localhost:3001/api-docs/openapi.json
```

This can be imported into tools like:
- Postman
- Insomnia
- API testing frameworks
- Code generators

### Using Swagger UI

The Swagger UI allows you to:

- 📖 Browse all available endpoints
- 🧪 Test API calls directly from the browser
- 📝 View request/response schemas
- 🔍 Search and filter endpoints by tags
- 💾 Download OpenAPI specification

**Endpoint Tags:**
- `accounts` - Account management operations
- `batches` - Batch management operations  
- `bets` - Bet status and update operations

---

## Local Development

### Backend
```bash
cargo run
```

Runs API on:
- **API**: `http://localhost:3001`
- **Swagger UI**: `http://localhost:3001/swagger-ui`
- **Health Check**: `http://localhost:3001/health`

### Frontend
```bash
cd betting-frontend
npm install
npm start
```

Runs UI on:
```
http://localhost:3000
```

React dev server proxies API and SSE requests to port 3001.

---

## Docker Setup
```bash
docker compose up --build
```

Services:

- **BetStream API** (Port 3001)
- **Frontend** (Port 3000)
- **Nginx** (for proxy/SSE)

### Accessing Services

After starting Docker:

| Service | URL |
|---------|-----|
| Frontend | `http://localhost:3000` |
| API | `http://localhost:3001` |
| Swagger UI | `http://localhost:3001/swagger-ui` |
| Health Check | `http://localhost:3001/health` |
| SSE Endpoint | `http://localhost:3001/sse` |

---

## Database Seeding

To initialize the database and create default accounts/batches:
```bash
./data/seed.sh
```

This script will:
- Create sample accounts
- Generate test batches
- Populate with example bets

---

## API Reference

All API endpoints are prefixed with `/api/v1`, unless stated otherwise.

> **💡 Tip:** For interactive API testing, use [Swagger UI](http://localhost:3001/swagger-ui)

### General Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | API root (health message) |
| `GET` | `/health` | Health check |
| `GET` | `/swagger-ui` | Interactive API documentation |
| `GET` | `/api-docs/openapi.json` | OpenAPI specification |

### Account Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/accounts` | List all accounts |
| `POST` | `/api/v1/accounts` | Create a new account |
| `GET` | `/api/v1/accounts/{id}` | Get account details |
| `PUT` | `/api/v1/accounts/{id}` | Update an account |
| `DELETE` | `/api/v1/accounts/{id}` | Delete an account |

### Batch Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/v1/accounts/{id}/batches` | Create a new batch for an account |
| `GET` | `/api/v1/accounts/{id}/batches` | Get all batches for an account |
| `DELETE` | `/api/v1/accounts/{id}/batches/{batch_id}` | Submit (complete) a batch |

### Bet Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PATCH` | `/api/v1/accounts/{id}/batches/{batch_id}/bets/{bet_id}` | Update a single bet status |
| `PATCH` | `/api/v1/accounts/{id}/batches/{batch_id}/bets` | Bulk update bet statuses |

### Server-Sent Events (SSE)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/sse` | Subscribe to real-time events |

**Event Types:**
- `account_created` - New account created
- `account_updated` - Account details modified
- `account_deleted` - Account removed
- `batch_created` - New batch created
- `batch_completed` - Batch submitted/completed
- `bet_status_updated` - Single bet status changed
- `batch_bets_updated` - Multiple bets updated
- `keep-alive` - Connection heartbeat (every 15s)

---

## Production Notes

### Nginx Configuration

If using Nginx as a reverse proxy, ensure proper configuration for SSE:
```nginx
location /sse {
    proxy_pass http://backend:3001;
    proxy_http_version 1.1;
    proxy_set_header Connection '';
    proxy_set_header X-Accel-Buffering no;
    proxy_buffering off;
    proxy_cache off;
    chunked_transfer_encoding off;
}

location /swagger-ui {
    proxy_pass http://backend:3001;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}

location /api-docs/ {
    proxy_pass http://backend:3001;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
}
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:./betstream.db?mode=rwc` | Database connection string |
| `CORS_ORIGIN` | `*` | Allowed CORS origins |

### Security Considerations

- 🔒 Consider adding authentication for production
- 🔒 Restrict Swagger UI access in production if needed
- 🔒 Use HTTPS for all endpoints
- 🔒 Configure proper CORS settings
- 🔒 Implement rate limiting

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add/update tests
5. Update documentation (including Swagger annotations)
6. Submit a pull request

---

## License

MIT License © 2026 nimeshpahadi

---

## Quick Links

- 📚 [Swagger UI](http://localhost:3001/swagger-ui) - Interactive API Documentation
- 📄 [OpenAPI Spec](http://localhost:3001/api-docs/openapi.json) - Machine-readable API definition
- 🔥 [Live Demo](#) - Coming soon
- 📖 [Full Documentation](#) - Coming soon

---

**Made with ❤️ using Rust, Axum, React, and Swagger**
