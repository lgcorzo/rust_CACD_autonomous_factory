---
iso_doc_type: "Procedure"
iso_viewpoint: "QualityView"
type: "user_guide"
title: "ISO 26514 Developer & User Guide"
description: "Step-by-step instructions for building, testing, benchmark execution, and extending MCP tools."
tags: ["iso26514", "developer_guide", "building", "testing"]
timestamp: "2026-07-31T16:35:00Z"
---

# ISO 26514 Developer & User Guide

## 1. Prerequisites & Environment Setup

- **Rust Toolchain**: Rust 1.75+ (`rustc`, `cargo`)
- **Python**: Python 3.10+ (for local AST extraction scripts)
- **Local AST Tool**: `graphify` (`pip install graphifyy` or `uv pip install graphifyy`)

---

## 2. Building the Project

Compile the full Cargo workspace:
```bash
cargo build --workspace --release
```

To compile specific binary targets:
```bash
# Build the MCP server
cargo build --bin factory-mcp-server

# Build the CLI tool
cargo build --bin factory-cli
```

---

## 3. Running Unit & Integration Tests

Run the complete test suite across all 5 workspace crates:
```bash
cargo test --workspace
```

Run specific crate tests:
```bash
# Security & zeroize tests
cargo test -p factory-core --test security_tests

# Kafka integration tests
cargo test -p factory-infrastructure --test kafka_integration

# gVisor sandbox integration tests
cargo test -p factory-mcp-server --test gvisor_integration
```

---

## 4. Running Benchmarks

Execute zeroize security benchmarks:
```bash
cargo bench -p factory-core --bench zeroize_benchmark
```

---

## 5. Updating OpenWiki Architecture Documentation

Whenever code changes are made to `crates/`, update the OpenWiki knowledge graph:
```bash
# 1. Update AST graph
graphify update .

# 2. Run OpenWiki AST generator
python3 generate_openwiki.py
```
