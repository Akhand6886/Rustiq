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
