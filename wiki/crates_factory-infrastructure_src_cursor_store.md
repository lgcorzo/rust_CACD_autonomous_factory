# factory-infrastructure::cursor_store — Polling State

> **Source**: `crates/factory-infrastructure/src/cursor_store.rs`
> **Layer**: Infrastructure

---

Cursor store for persisting poller state across restarts. Tracks the last processed issue/PR/pipeline event ID per repository to avoid re-processing.

---

> *Related: [lib.rs](crates_factory-infrastructure_src_lib.md) · [Infrastructure Adapters](INFRASTRUCTURE-ADAPTERS.md)*
