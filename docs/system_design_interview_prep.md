# Rustiq System Design Interview Preparation

This document lists common systems questions and design principles covered by Rustiq.

## 1. Push-based vs. Pull-based Messaging Systems

| Architecture | Push-based (e.g. Webhooks) | Pull-based (e.g. Rustiq, SQS) |
|---|---|---|
| **Flow Control** | Managed by sender; risk of overwhelming receiver | Managed by receiver; natural backpressure |
| **Worker Health** | Requires active heartbeat or endpoint | Worker pulls only when it has capacity |
| **Implementation** | Simpler client, complex load balancers | Workers pull from broker; state stored in DB |
