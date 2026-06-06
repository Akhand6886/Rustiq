# Rustiq System Architecture Blueprint

This document describes the high-level architecture, component boundaries, lifecycles, and data flows of Rustiq—a production-grade distributed task queue built in Rust.

## 1. Component Boundaries

The system is composed of four decoupled layers communicating via defined protocol boundaries:
- **Producer Client**
- **Broker / Server**
- **Storage Layer**
- **Worker Pool**

### 1.1 Producer Client
- **Role:** Generates tasks, serializes payloads, assigns unique identifiers (UUIDs), and dispatches requests to the Broker.
- **Communication Protocol:** HTTP REST API (using JSON) or high-performance TCP.
- **Crates/Tools:** `serde` (serialization), `uuid` (unique IDs), `reqwest` (HTTP requests).

### 1.2 Broker / Server
- **Role:** Acts as the central coordinator. Receives jobs from Producers, manages queue state, schedules delayed tasks, and dispatches jobs to Workers.
- **Crates/Tools:** `tokio` (async runtime), `axum` (web framework).

### 1.3 Storage Layer
- **Role:** Guarantees durability of jobs, records state changes, logs execution results, and manages dead-letter assignments.
- **Crates/Tools:** `sled` (embedded key-value DB), `redis` (clustered memory storage).

### 1.4 Worker Pool
- **Role:** Multi-threaded execution environment. Pulls jobs from the Broker, routes tasks to handlers, manages panics, and reports success or failures.
- **Crates/Tools:** `tokio` (concurrency tasks), `async-trait`.
