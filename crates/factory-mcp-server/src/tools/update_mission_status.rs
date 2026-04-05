use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::fs::{OpenOptions, File};
use std::io::Write;
use chrono::Local;

pub struct UpdateMissionStatusTool {
    docs_path: String,
}

impl UpdateMissionStatusTool {
    pub fn new(docs_path: String) -> Self {
        Self { docs_path }
    }
}

#[async_trait]
impl Tool for UpdateMissionStatusTool {
    fn name(&self) -> String {
        "update_mission_status".to_string()
    }

    fn description(&self) -> String {
        "Updates project documentation and mission history after a mission or testing phase.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "mission_id": {"type": "string"},
                "status": {"type": "string", "enum": ["Success", "Failure", "In Progress"]},
                "summary": {"type": "string"},
                "artifacts": {"type": "array", "items": {"type": "string"}},
                "agent_name": {"type": "string"}
            },
            "required": ["mission_id", "status", "summary"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let mission_id = params["mission_id"].as_str().unwrap_or("unknown");
        let status = params["status"].as_str().unwrap_or("Unknown");
        let summary = params["summary"].as_str().unwrap_or("");
        let agent_name = params["agent_name"].as_str().unwrap_or("OpenCodeAgent");
        let artifacts = params["artifacts"].as_array()
            .map(|a| a.iter().map(|v| v.as_str().unwrap_or("")).collect::<Vec<_>>().join(", "))
            .unwrap_or_default();

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 1. Update docs/mission_history.md (Terminal-friendly table)
        let history_path = format!("{}/mission_history.md", self.docs_path);
        let mut file = OpenOptions::new()
            .append(true)
            .open(&history_path)?;

        let new_entry = format!(
            "| {} | {} | {} | {} | {} | (Agent: {}) |\n",
            mission_id, timestamp, status, summary, artifacts, agent_name
        );
        file.write_all(new_entry.as_bytes())?;

        // 2. Update a detailed summary file for the specific mission
        let summary_path = format!("{}/mission_{}_summary.md", self.docs_path, mission_id);
        let mut summary_file = File::create(&summary_path)?;
        let summary_content = format!(
            "# Mission Summary: {}\n\n- **Status**: {}\n- **Date**: {}\n- **Agent**: {}\n\n## Summary\n{}\n\n## Artifacts\n{}\n",
            mission_id, status, timestamp, agent_name, summary, artifacts
        );
        summary_file.write_all(summary_content.as_bytes())?;

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: format!("Mission {} updated in documentation.", mission_id),
            }],
            is_error: false,
        })
    }
}
