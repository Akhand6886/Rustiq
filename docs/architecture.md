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
