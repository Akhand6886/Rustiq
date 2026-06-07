# Advanced Async Rust Concurrency Review

This document contains detailed review notes, system design comparisons, and code analysis focusing on Rust's async runtime model and thread safety.

## 1. Concurrency Models Overview

Concurrency allows programs to execute out-of-order or in parallel, maximizing resource utilization. Different environments handle concurrency differently:
- OS Threads
- Green Threads (M:N scheduling)
- Async / Await (Event-driven poll-based futures)
