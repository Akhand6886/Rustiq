# Advanced Async Rust Concurrency Review

This document contains detailed review notes, system design comparisons, and code analysis focusing on Rust's async runtime model and thread safety.

## 1. Concurrency Models Overview

Concurrency allows programs to execute out-of-order or in parallel, maximizing resource utilization. Different environments handle concurrency differently:
- OS Threads
- Green Threads (M:N scheduling)
- Async / Await (Event-driven poll-based futures)

### 1.1 Threads vs. Async
- **OS Threads:** High memory overhead (typically 2MB stack per thread). Context switches require kernel transitions, which are relatively slow.
- **Green Threads (Go):** Managed by runtime scheduler. Small stack overhead (starts at 2KB), but requires runtime overhead.
- **Async/Await (Rust):** Zero-cost abstraction. Compile-time transformation into state machines. No runtime overhead until an executor is introduced. Stack sizes are determined at compile time.

### 1.2 Cooperative Multitasking
Rust's async tasks are cooperative. This means a running task must yield control back to the executor (via `.await`) to allow other tasks to run. If an async task runs a long computation without yielding, it will block the execution thread.

## 2. Runtimes, Futures, and Executors

Rust's standard library provides the interface for async coding (`Future` trait), but does not provide an execution engine. An **Executor** is required to run futures.

### 2.1 The Poll Model
Unlike javascript promises, which start executing immediately when created, Rust futures are lazy. They do nothing until polled. The `Future` trait defines:
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
