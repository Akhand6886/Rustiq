# Sled Embedded Storage Architecture

Rustiq leverages **`sled`** as its primary persistent storage backend. `sled` is a pure-Rust, embedded transactional database designed for low latency, high throughput, and simplicity. This document outlines the mechanical study of `sled` and explains why it fits perfectly within our distributed task queue architecture.

## Why Embedded Storage? (Sled vs. PostgreSQL/Redis)

For a task queue pushing 10,000+ jobs per second, network hop latency is a significant bottleneck. 
- **No Network Hop**: Because `sled` is embedded directly into the Rustiq binary, there is zero network serialization overhead or connection pooling management required.
- **ACID Guarantees**: Unlike some embedded key-value stores, `sled` guarantees Atomicity, Consistency, Isolation, and Durability out-of-the-box.
- **Lock-free Concurrency**: `sled` utilizes a lock-free multi-version concurrency control (MVCC) mechanism, preventing writer starvation when a large batch of readers is scanning the queue.

## Mechanics: B-Trees and Key Spaces

`sled` operates as a concurrent Bw-Tree (a lock-free B-tree variant). Data is stored as opaque byte slices (`[u8]`).

### Data Layout Strategy
To represent our queues and jobs efficiently, we must design a strict key schema:
- **Jobs**: `job:<uuid>` -> JSON bytes of the `Job` struct.
- **Queues (Future Indexing)**: `queue:<queue_name>:<uuid>` -> Empty bytes (used to quickly iterate over jobs in a specific queue without scanning the entire database).

By leveraging `sled`'s ordered keys, prefix scanning (e.g., iterating over all keys starting with `queue:email:`) becomes highly efficient.

## Recovery Mechanisms and Memory-Mapped Files

`sled` maintains high performance and durability through a log-structured architecture:
- **Memory-Mapped I/O**: The database uses memory-mapped files to allow the OS virtual memory manager to handle caching seamlessly. This prevents double-caching and reduces memory pressure in userspace.
- **Write-Ahead Logging (WAL)**: All transactions are appended to a write-ahead log. When the system crashes or restarts unexpectedly, `sled` replays this log to reconstruct the B-Tree state exactly as it was.
- **Clean Shutdowns vs. Crash Recovery**: `sled::Db::flush()` can be used to ensure all WAL buffers are synced to disk. However, even if `flush` is not called due to a panic or power loss, the log-structured nature ensures the database will recover gracefully up to the last synced transaction on the next instantiation (`sled::open()`).

## Concurrency Control (MVCC)

Multi-Version Concurrency Control (MVCC) allows readers to view a consistent snapshot of the database while writers are actively appending new data. 
- A background worker pulling 50 jobs from a queue will not block an API request saving a new job.
- Updates to existing jobs (e.g., status transitions from `Queued` to `Processing`) use `compare_and_swap` mechanics to prevent race conditions across concurrent workers.
