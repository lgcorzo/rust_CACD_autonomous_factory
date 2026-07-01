use crate::protocol::CallToolResult;
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

pub struct BridgeTool;

impl BridgeTool {
    fn get_checkpoint_path(mission_id: &str) -> PathBuf {
        PathBuf::from(format!("/tmp/bridge_state_{}.json", mission_id))
    }

    pub fn load_state(mission_id: &str) -> anyhow::Result<Value> {
        let path = Self::get_checkpoint_path(mission_id);
        if !path.exists() {
            return Ok(json!({
                "status": "not_found",
                "message": format!("No checkpoint found for mission {}", mission_id)
            }));
        }

        let content = fs::read_to_string(&path)?;
        let state: Value = serde_json::from_str(&content)?;
        Ok(state)
    }

    pub fn save_state(mission_id: &str, state: Value) -> anyhow::Result<Value> {
        let path = Self::get_checkpoint_path(mission_id);
        let content = serde_json::to_string_pretty(&state)?;
        fs::write(&path, content)?;

        Ok(json!({
            "status": "success",
            "message": format!("Checkpoint saved for mission {}", mission_id)
        }))
    }
}

#[async_trait]
impl Tool for BridgeTool {
    fn name(&self) -> String {
        "sync_bridge_state".to_string()
    }

    fn description(&self) -> String {
        "Sync Spec-Kit and Superpowers state. Use action='load' to read the state or action='save' with a 'state' object to persist it.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "mission_id": {
                    "type": "string",
                    "description": "The mission ID to sync state for"
                },
                "action": {
                    "type": "string",
                    "enum": ["load", "save"],
                    "description": "The action to perform: load or save"
                },
                "state": {
                    "type": "object",
                    "description": "The state object to save (only required if action is 'save')"
                }
            },
            "required": ["mission_id", "action"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let mission_id = params
            .get("mission_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'mission_id'"))?;

        let action = params
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'action'"))?;

        let result = match action {
            "load" => Self::load_state(mission_id)?,
            "save" => {
                let state = params
                    .get("state")
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'state' object for save action"))?;
                Self::save_state(mission_id, state)?
            }
            _ => anyhow::bail!("Invalid action. Must be 'load' or 'save'"),
        };

        Ok(CallToolResult {
            content: vec![crate::protocol::McpContent::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }
}
