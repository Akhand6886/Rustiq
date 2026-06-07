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

### 2.2 Wakers and Event Notification
When a future is polled and cannot complete, it returns `Poll::Pending`. The executor registers a `Waker` in the context. When the underlying resource becomes available, the OS or driver signals the waker, which notifies the executor to schedule the task for another poll.

## 3. Tokio Runtime Internals

Tokio is the industry-standard multi-threaded async runtime for Rust. It uses a **Work-Stealing Scheduler** to balance workloads across threads:
- Each OS thread managed by Tokio has its own local run queue.
- If a thread's local run queue becomes empty, it attempts to steal tasks from other threads' queues.

### 3.1 Work-Stealing vs Single-Threaded
- **Work-Stealing (multi_thread):** Best for parallel execution on multi-core systems, typical for high-throughput network servers.
- **Current Thread (current_thread):** Runs all tasks on the current thread. Best for single-core microcontrollers or simple CLI tools, eliminating cross-thread synchronization overhead.

### 3.2 Thread Blocking inside Async Code
Blocking operations (like `std::thread::sleep` or sync file IO) inside async contexts block the local scheduler thread, preventing other queued tasks from executing.

### 3.3 Mitigation: `spawn_blocking`
To run blocking operations, use `tokio::task::spawn_blocking`, which moves the blocking execution to a dedicated pool of OS threads outside the core scheduler.
```rust
let result = tokio::task::spawn_blocking(move || {
    // Perform blocking synchronous operations here
    std::thread::sleep(std::time::Duration::from_secs(1));
}).await?;
```

## 4. Send and Sync Concurrency Boundaries

Rust guarantees thread safety at compile time using markers:
- `Send`: Indicates a type can be transferred across thread boundaries safely.
- `Sync`: Indicates it is safe to share references to a type across thread boundaries.

### 4.1 Thread Safety in Future Spawning
When you spawn a task on a multi-threaded executor (`tokio::spawn`), the future must be `Send` because it may be moved and run on a different thread. Any variables captured by the future or held across `.await` points must also implement `Send`.

### 4.2 Compile-Time Data Race Prevention
Data races occur when two threads access the same memory location concurrently, and at least one access is a write. Rust's borrow checker enforces ownership rules, preventing shared mutable state without synchronization.

## 5. Thread-Safe Sharing Mechanisms

To share resources across async task boundaries, use thread-safe pointer types:
- `Arc<T>`: Atomic reference counting wrapper. Allows multiple threads to own shared access to an immutable resource.

### 5.1 Synchronization Types
- `Mutex<T>`: Mutual exclusion lock. Allows only one thread to access the resource at a time.
- `RwLock<T>`: Reader-writer lock. Allows multiple readers or one writer. Best for read-heavy operations.
*Note:* Always use async-aware synchronization (e.g. `tokio::sync::Mutex`) when holding locks across `.await` points to avoid blocking scheduler threads.
