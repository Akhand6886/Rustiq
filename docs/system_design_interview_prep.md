# Rustiq System Design Interview Preparation

This document lists common systems questions and design principles covered by Rustiq.

## 1. Push-based vs. Pull-based Messaging Systems

| Architecture | Push-based (e.g. Webhooks) | Pull-based (e.g. Rustiq, SQS) |
|---|---|---|
| **Flow Control** | Managed by sender; risk of overwhelming receiver | Managed by receiver; natural backpressure |
| **Worker Health** | Requires active heartbeat or endpoint | Worker pulls only when it has capacity |
| **Implementation** | Simpler client, complex load balancers | Workers pull from broker; state stored in DB |

## 2. At-Least-Once Delivery Guarantees

Rustiq achieves at-least-once delivery through:
- **Lease model:** A job remains in the persistent storage even after dispatch.
- **ACK protocol:** Workers must explicitly acknowledge completion. If they don't, the job is requeued.
