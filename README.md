# Rust RocksDB Blockchain State Store

## Overview
This project implements a **key-value blockchain state store** in **Rust** using **RocksDB**. It is designed to store both **current and archival blockchain state** efficiently and support **high-throughput parallel read/write operations**. The system leverages **async processing with Tokio** for concurrency and includes **parallelizable data structures** for fast access.

The project simulates low-level blockchain state management similar to what is needed for MoveVM-based chains and Ethereum-like nodes.

## Features
- Async processing with **Tokio runtime**.
- Key-value storage using **RocksDB**.
- Parallel read/write operations for high throughput.
- Fault-tolerant and scalable storage.
- Benchmarking for latency and throughput.

## Technologies Used
- **Rust** – System programming language for performance and safety.
- **RocksDB** – Embedded key-value store for blockchain state.
- **Tokio** – Asynchronous runtime for concurrent operations.

## Getting Started

### Prerequisites
- Rust >= 1.70
- Cargo
- RocksDB library (system dependency)

### Installation
1. Clone the repository:
```bash
git clone https://github.com/<your-username>/rust-rocksdb-blockchain.git
cd rust-rocksdb-blockchain
