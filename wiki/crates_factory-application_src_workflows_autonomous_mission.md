---
type: module
title: "autonomous_mission.rs"
source_path: "crates/factory-application/src/workflows/autonomous_mission.rs"
description: "Documentation for crates/factory-application/src/workflows/autonomous_mission.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# autonomous_mission.rs

Source File: `crates/factory-application/src/workflows/autonomous_mission.rs`

## Component Architecture

```mermaid
classDiagram
    class MissionInput
    class MissionOutput
```

## Execution Flow

```mermaid
flowchart TD
    Start --> from_protobuf
    from_protobuf --> create_mission_workflow
    create_mission_workflow --> test_mission_input_from_protobuf
    test_mission_input_from_protobuf --> End
```
