# In-Memory Database Project TODO List

This document tracks the progress of building an in-memory key-value database in Rust. Check off each item as you complete it!

---

- [x] **1. Project Setup**
    - [x] Initialize a new Rust project
    - [x] Set up the recommended directory structure (src/, tests/, etc.)
    - [x] Add a README.md describing your project goals

- [ ] **2. Core Data Structure**
    - [ ] Design and implement the basic in-memory key-value store (e.g., using HashMap)
    - [ ] Implement basic operations: SET, GET, DELETE

- [ ] **3. Command-Line Interface (CLI)**
    - [ ] Create a simple CLI to interact with your DB (REPL: Read-Eval-Print Loop)
    - [ ] Parse and execute commands

- [ ] **4. Persistence (Optional, Intermediate)**
    - [ ] Implement snapshotting (save/load to disk)
    - [ ] (Optional) Add append-only log (AOF) for durability

- [ ] **5. Networking**
    - [ ] Add TCP server support so clients can connect over the network
    - [ ] Handle multiple clients (basic concurrency)

- [ ] **6. Protocol**
    - [ ] Design a simple text-based protocol (like Redis RESP or Memcached)
    - [ ] Parse incoming requests and send responses

- [ ] **7. Expiry and Eviction**
    - [ ] Add support for key expiry (TTL)
    - [ ] (Optional) Implement basic eviction policies (e.g., LRU)

- [ ] **8. Testing & Benchmarking**
    - [ ] Write unit and integration tests for all components
    - [ ] Benchmark performance (compare to HashMap baseline)

- [ ] **9. Documentation & Polish**
    - [ ] Document public APIs using rustdoc
    - [ ] Update README with usage examples
    - [ ] (Optional) Add Dockerfile or deployment instructions

---

Update this file as you complete each step to track your progress!
