# Rustiq Architecture and Documentation

Welcome to the Rustiq project documentation. This folder contains detailed documentation about the system design, components, lifecycles, and data flows of Rustiq.

## Documents
- [Architecture Blueprint](architecture.md)
- [System Design Interview Prep](system_design_interview_prep.md)
- [Sled Storage Architecture](sled_architecture.md)

## Learning Resources
- *Designing Data-Intensive Applications* by Martin Kleppmann (Chapters 7, 8, 11)
- *Programming Rust* (Concurrency and Async)

## Day 1 & 2 Checklists Complete
- System Design Boundaries Defined
- Job Lifecycles Documented
- Sequence Flows Verified
- [Async Rust Concurrency Review](async_rust_review.md)

## Day 3 Checklist Complete
- Binary Cargo project initialized in root
- Core production dependencies pinned in Cargo.toml
- Standard logging subscriber configured in main.rs
- [Workspace Setup and Dependencies](workspace.md)

## Day 4 Checklist Complete
- Core domain model structs defined in [types.rs](file:///Users/alpha/Desktop/Project%20report/Rustiq/src/types.rs)
- Implemented `JobStatus` enum (Queued, Processing, Done, Failed, DeadLetter)
- Implemented `Job` struct containing task schemas, timing metadata, execution results, and errors
- Created `Job::new` helper constructors and status checkers
- Verified serialization/deserialization boundaries via unit tests

## Day 5 Checklist Complete
- Defined `QueueConfig` struct to manage customizable, queue-specific metrics
- Implemented `QueueConfigBuilder` supporting fluid configuration paths
- Created helper methods `apply_visibility_timeout`, `has_exceeded_retries`, and `from_json`
- Integrated defaults and builders backed by a comprehensive unit test suite in [types.rs](file:///Users/alpha/Desktop/Project%20report/Rustiq/src/types.rs)
- Documented configuration schema additions in [workspace.md](workspace.md)

## Day 6 Checklist Complete
- Created [errors.rs](file:///Users/alpha/Desktop/Project%20report/Rustiq/src/errors.rs) to centralize error handling
- Defined `RustiqError` enum leveraging `thiserror` for descriptive message formats
- Implemented `From` conversions for `serde_json::Error` and `uuid::Error`
- Added the `is_recoverable()` classification helper to isolate database/storage faults from terminal payload configuration mismatches
- Covered error transitions and formatting bounds via comprehensive unit tests

## Day 7 Checklist Complete
- Defined the asynchronous `Storage` trait with standard queue database operations (`save_job`, `get_job`, `delete_job`, `update_job_status`, `get_jobs_by_queue`, `get_all_jobs`, `clear_queue`)
- Implemented the thread-safe, in-memory `MockStorage` using `Arc<RwLock<HashMap<Uuid, Job>>>`
- Covered database operations, updates, and filtering behaviors via a comprehensive unit test suite in [storage/mod.rs](../src/storage/mod.rs)
- Documented storage abstraction architectures and implementation layers in [workspace.md](workspace.md)

## Day 9 & 10 Checklist Complete
- Migrated tracing and `tracing_subscriber` logic into a central, dedicated [logging.rs](../src/logging.rs) module
- Instrumented the `Storage` trait implementations with `#[instrument]` macros and detailed `debug!` hooks to observe internal queries
- Validated application formatting comprehensively via `cargo fmt`
- Resolved codebase lint warnings raised by `cargo clippy`
- Updated system design indexing via [workspace.md](workspace.md) and [README.md](README.md)

## Day 11 & 12 Checklist Complete
- Studied `sled` mechanics (B-tree structure, key spaces, MVCC, write-ahead logs)
- Outlined key-value mapping strategies (e.g. `job:<uuid>`)
- Documented Sled's advantages and architectures in [sled_architecture.md](sled_architecture.md)
- Set up `SledStorage` struct wrapping `sled::Db`
- Covered basic Sled initialization via unit tests
