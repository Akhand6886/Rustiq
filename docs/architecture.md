# Rustiq System Architecture Blueprint

This document describes the high-level architecture, component boundaries, lifecycles, and data flows of Rustiq—a production-grade distributed task queue built in Rust.

## 1. Component Boundaries

The system is composed of four decoupled layers communicating via defined protocol boundaries:
- **Producer Client**
- **Broker / Server**
- **Storage Layer**
- **Worker Pool**
