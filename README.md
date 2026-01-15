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
- [Renaming from manualbettingserver](#renaming-from-manualbettingserver)

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

[Nginx / Traefik] → [BetStream API + Frontend]


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

Nginx (optional, for proxy/SSE)

Database Seeding

To initialize the database and create default accounts/batches:
./data/seed.sh

API Reference

All API endpoints are prefixed with /api/v1/accounts.

Examples:

GET /api/v1/accounts — List accounts

GET /api/v1/accounts/{id} — Get account details

GET /api/v1/accounts/{id}/batches — Get account batches

PATCH /api/v1/accounts/{id}/batches/{batch_id}/bets/{bet_id} — Update bet status

DELETE /api/v1/accounts/{id}/batches/{batch_id} — Submit a batch

SSE endpoint:
GET /sse

License

MIT License © 2026 nimeshpahadi
