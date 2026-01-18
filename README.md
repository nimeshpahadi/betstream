# BetStream 🚀

[![Rust](https://img.shields.io/badge/Rust-1.72+-orange?logo=rust)](https://www.rust-lang.org/) 
[![React](https://img.shields.io/badge/React-18-blue?logo=react)](https://reactjs.org/)
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

- Account-based bet management
- Batch processing workflow
- Real-time updates via Server-Sent Events (SSE)
- Interactive operator web UI
- Manual and programmatic bet updates
- Persistent storage with SQLite (or extendable to Postgres)
- Dockerized development and production environment

## Architecture

---

+-----------------+ REST / SSE +------------------------+
| | <---------------------------> | |
| React UI | | BetStream API (Rust) |
| (Frontend) | | + Axum / SQLx |
| | | |
+-----------------+ +------------------------+
|
v
+----------------+
| SQLite / Postgres |
| Accounts, Batches |
| Bets |
+----------------+

[Nginx] → [BetStream API + Frontend]


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

Created → Active → Submitted → Completed


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
- `account_deleted`
- `batch_created`
- `batch_completed`
- `bet_status_updated`
- `ping`

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

## Local Development

### Backend

```bash
cargo run


Runs API on:
http://localhost:3001
cd betting-frontend
npm install
npm start
Runs UI on:
http://localhost:3000
React dev server proxies API and SSE requests to port 3001.

Docker Setup
docker compose up --build

Services:

BetStream API

Frontend

Nginx (for proxy/SSE)

Database Seeding

To initialize the database and create default accounts/batches:
./data/seed.sh

API Reference

All API endpoints are prefixed with /api/v1/accounts, unless stated otherwise.

General Endpoints

GET / — API root (health message)

GET /health — Health check

Account Endpoints

GET /api/v1/accounts
List all accounts

POST /api/v1/accounts
Create a new account

GET /api/v1/accounts/{id}
Get account details

PUT /api/v1/accounts/{id}
Update an account

DELETE /api/v1/accounts/{id}
Delete an account

Batch Endpoints

POST /api/v1/accounts/{id}/batches
Create a new batch for an account

GET /api/v1/accounts/{id}/batches
Get all batches for an account

DELETE /api/v1/accounts/{id}/batches/{batch_id}
Submit (complete) a batch

Bet Endpoints

PATCH /api/v1/accounts/{id}/batches/{batch_id}/bets/{bet_id}
Update a single bet status within a batch

PATCH /api/v1/accounts/{id}/batches/{batch_id}/bets
Bulk update bet statuses within a batch

Server-Sent Events (SSE)

GET /sse
Subscribe to real-time events, including:

Account created / deleted

Batch created / completed

Bet status updates

Keep-alive ping events

License

MIT License © 2026 nimeshpahadi
