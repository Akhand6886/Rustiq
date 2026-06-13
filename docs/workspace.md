# Rustiq Project Workspace & Dependency Architecture

This document tracks the workspace layout, pinned dependencies, and tooling selections configured on **Day 3**.

## 📁 Project Structure

```
Rustiq/
├── Cargo.toml               # Cargo package manifest & release profiles
├── rust-toolchain.toml      # Pinned rustc and utility version
├── docs/
│   ├── README.md            # Docs index
│   └── workspace.md         # Workspace documentation [This file]
└── src/
    ├── main.rs              # Application entrypoint & tracing setup
    ├── types.rs             # Core task queue domain types (Days 4 & 5)
    ├── errors.rs            # Custom error types (Day 6)
    ├── storage/
    │   └── mod.rs           # DB interface trait and mock storage (Day 7-8)
    ├── api/
    │   └── mod.rs           # HTTP Router & route handlers (Day 23)
    └── worker/
        └── mod.rs           # Background worker loop & registry (Day 37)
```

## 📦 Selected Dependencies

We utilize a modern, modular, production-grade stack to ensure high throughput, reliability, and structured visibility:

| Crate | Version | Key Features / Purpose |
| :--- | :--- | :--- |
| **`tokio`** | `1.35` | Multi-threaded async runtime with full feature sets |
| **`axum`** | `0.7` | High-performance, ergonomic web routing |
| **`serde`** | `1.0` | Serialization and deserialization framework (with `derive`) |
| **`serde_json`** | `1.0` | Parsing and handling of JSON payloads |
| **`uuid`** | `1.6` | Task and client UUID generation (with `v4`, `serde`) |
| **`chrono`** | `0.4` | Timestamp formatting and database time boundaries |
| **`sled`** | `0.34` | Pure-Rust transactional embedded database |
| **`tracing`** | `0.1` | Application logging facade |
| **`tracing-subscriber`** | `0.3` | JSON logging & environment filter configuration |
| **`async-trait`** | `0.1` | Support for asynchronous trait interfaces |
| **`thiserror`** | `1.0` | Helper macros to simplify error definitions |

## ⚙️ Compilation Profile Configurations

To achieve minimal latency and high throughput (target > 10,000 requests/sec):
- **Optimization level** is set to `3` for release builds.
- **Link-Time Optimization (LTO)** is enabled (`lto = true`) to allow cross-crate optimizations.
- **Codegen units** are set to `1` to maximize optimization range across translation units.

## 🔧 Queue Configurations (Day 5 additions)

The core domain model types in `src/types.rs` now include queue-specific parameters:
- **`QueueConfig`**: Struct holding configuration metrics such as `visibility_timeout_secs`, `max_retries`, `max_concurrency`, and `dead_letter_queue` name.
- **`QueueConfigBuilder`**: Implement builder patterns to construct queue configurations cleanly.
- **Lease & Retry Helpers**: Methods like `apply_visibility_timeout` and `has_exceeded_retries` allow applying queue settings dynamically to individual jobs.

## ⚠️ Error Handling Architecture (Day 6 additions)

The central error handling system in `src/errors.rs` is defined by:
- **`RustiqError`**: Enum deriving `thiserror::Error` for descriptive error handling:
  - `StorageError`: For database, I/O, or connection failures (classified as **recoverable**).
  - `QueueNotFound`: For missing queue names (classified as **terminal**).
  - `SerializationError`: For JSON structure or parsing mistakes (classified as **terminal**).
  - `InvalidPayload`: For payload content validation failures (classified as **terminal**).
  - `JobNotFound`: For missing job UUIDs in database operations (classified as **terminal**).
- **Conversions**: Out-of-the-box `From` conversions for common external error types:
  - `serde_json::Error` -> `RustiqError::SerializationError`
  - `uuid::Error` -> `RustiqError::InvalidPayload`
- **Recoverability Checks**: Method `is_recoverable()` determines whether processing should retry or fail immediately.


