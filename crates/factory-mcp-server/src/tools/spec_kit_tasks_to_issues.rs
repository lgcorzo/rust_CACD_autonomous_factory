use crate::protocol::CallToolResult;
use crate::tools::Tool;
use async_trait::async_trait;
use factory_infrastructure::GitlabClient;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct SpecKitTasksToIssuesTool {
    gitlab_client: Arc<dyn GitlabClient>,
}

impl SpecKitTasksToIssuesTool {
    pub fn new(gitlab_client: Arc<dyn GitlabClient>) -> Self {
        Self { gitlab_client }
    }
}

#[async_trait]
impl Tool for SpecKitTasksToIssuesTool {
    fn name(&self) -> String {
        "speckit_taskstoissues".to_string()
    }

    fn description(&self) -> String {
        "Parse tasks.md from the spec directory and create GitLab issues with Resource Limits."
            .to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "project_id": {
                    "type": "string",
                    "description": "The GitLab project ID or URL-encoded path (e.g., 'group%2Fproject')"
                }
            },
            "required": ["project_id"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let project_id = params
            .get("project_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'project_id'"))?;

        // 1. Determine specs directory
        let mut target_spec_dir = None;

        let config_str = std::fs::read_to_string(".specify/init-options.json").ok();
        let config: Option<serde_json::Value> =
            config_str.and_then(|s| serde_json::from_str(&s).ok());
        if let Some(specs_dir) = config
            .as_ref()
            .and_then(|c| c.get("specs_dir"))
            .and_then(|v| v.as_str())
        {
            let path = std::path::PathBuf::from(specs_dir);
            if path.exists() {
                target_spec_dir = Some(path);
            }
        }

        if target_spec_dir.is_none() {
            let entries = std::fs::read_dir("specs").ok();
            if let Some(entries) = entries {
                let mut dirs: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                dirs.sort_by_key(|dir| {
                    dir.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                });
                if let Some(latest) = dirs.last() {
                    target_spec_dir = Some(latest.path());
                }
            }
        }

        let spec_dir = match target_spec_dir {
            Some(dir) => dir,
            None => {
                return Ok(CallToolResult {
                    content: vec![crate::protocol::McpContent::Text {
                        text: "Error: No specs directory found.".to_string(),
                    }],
                    is_error: true,
                });
            }
        };

        // 2. Read tasks.md
        let tasks_path = spec_dir.join("tasks.md");
        let tasks_content = match std::fs::read_to_string(&tasks_path) {
            Ok(content) => content,
            Err(e) => {
                return Ok(CallToolResult {
                    content: vec![crate::protocol::McpContent::Text {
                        text: format!("Error reading tasks.md: {}", e),
                    }],
                    is_error: true,
                });
            }
        };

        // 3. Parse tasks
        let mut created_issues = Vec::new();
        for line in tasks_content.lines() {
            let line = line.trim();
            // Match "- [ ] Task description" or "* [ ] Task description"
            if line.starts_with("- [ ] ") || line.starts_with("* [ ] ") {
                let task_title = line[6..].trim();
                if task_title.is_empty() {
                    continue;
                }

                // 4. Create GitLab Issue with Resource Limit
                let description = format!("{}\n\n[RESOURCE_LIMIT: RAM <= 30Mi]", task_title);

                match self
                    .gitlab_client
                    .create_issue(project_id, task_title, &description)
                    .await
                {
                    Ok(issue) => {
                        created_issues
                            .push(format!("Created issue #{}: {}", issue.iid, issue.web_url));
                    }
                    Err(e) => {
                        return Ok(CallToolResult {
                            content: vec![crate::protocol::McpContent::Text {
                                text: format!(
                                    "Error creating GitLab issue for '{}': {}",
                                    task_title, e
                                ),
                            }],
                            is_error: true,
                        });
                    }
                }
            }
        }

        let output = if created_issues.is_empty() {
            "No uncompleted tasks found in tasks.md.".to_string()
        } else {
            format!(
                "Successfully created {} issues:\n{}",
                created_issues.len(),
                created_issues.join("\n")
            )
        };

        Ok(CallToolResult {
            content: vec![crate::protocol::McpContent::Text { text: output }],
            is_error: false,
        })
    }
}
