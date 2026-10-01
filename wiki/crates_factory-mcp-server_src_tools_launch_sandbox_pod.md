# factory-mcp-server::tools::launch_sandbox_pod — K8s Job Creation

> **Source**: `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`
> **Layer**: Interface

---

Sandbox pod launch tool: creates gVisor-sandboxed K8s Jobs with resource constraints (30 MiB/0.25 CPU app, 20 MiB/0.10 CPU sidecar, no egress).

---

> *Related: [lib.rs](crates_factory-mcp-server_src_lib.md) · [Tactical Design](TACTICAL-DESIGN.md)*
