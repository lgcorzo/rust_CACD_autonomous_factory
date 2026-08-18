use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const DEFAULT_FACTORY_MODEL: &str = "ollama/qwen2.5:7b";
pub const DEFAULT_PLANNER_MODEL: &str = "gpt-oss-120b";

/// Configuration mapping agents and tools to their respective LLM model endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentModelConfig {
    #[serde(default = "default_factory_model")]
    pub default_model: String,
    #[serde(default = "default_planner_model")]
    pub planner_model: String,
    #[serde(default)]
    pub agents: HashMap<String, String>,
}

fn default_factory_model() -> String {
    std::env::var("LITELLM_MODEL").unwrap_or_else(|_| DEFAULT_FACTORY_MODEL.to_string())
}

fn default_planner_model() -> String {
    std::env::var("LITELLM_PLANNER_MODEL")
        .or_else(|_| std::env::var("PLANNER_MODEL"))
        .unwrap_or_else(|_| DEFAULT_PLANNER_MODEL.to_string())
}

impl Default for AgentModelConfig {
    fn default() -> Self {
        Self {
            default_model: default_factory_model(),
            planner_model: default_planner_model(),
            agents: HashMap::new(),
        }
    }
}

impl AgentModelConfig {
    /// Load configuration from file or fallback to environment variables and default constants.
    pub fn load() -> Self {
        // 1. Check custom path from env var FACTORY_MODELS_CONFIG
        if let Ok(path_str) = std::env::var("FACTORY_MODELS_CONFIG") {
            if let Ok(cfg) = Self::from_file(&path_str) {
                return cfg;
            }
        }

        // 2. Candidate default file paths
        let candidates = [
            "config/models.yaml",
            "config/models.yml",
            "config/models.json",
            "models.yaml",
            "models.json",
            "../config/models.yaml",
            "../../config/models.yaml",
        ];

        for path in &candidates {
            if Path::new(path).exists() {
                if let Ok(cfg) = Self::from_file(path) {
                    return cfg;
                }
            }
        }

        // 3. Fallback to env variables / defaults
        Self::default()
    }

    /// Parse configuration from a specific YAML or JSON file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        let ext = path
            .as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mut config: AgentModelConfig = if ext == "json" {
            serde_json::from_str(&content)?
        } else {
            serde_yaml::from_str(&content)?
        };

        // If environment variable explicitly overrides default or planner, apply them
        if let Ok(env_default) = std::env::var("LITELLM_MODEL") {
            if !env_default.is_empty() {
                config.default_model = env_default;
            }
        }
        if let Ok(env_planner) =
            std::env::var("LITELLM_PLANNER_MODEL").or_else(|_| std::env::var("PLANNER_MODEL"))
        {
            if !env_planner.is_empty() {
                config.planner_model = env_planner;
            }
        }

        Ok(config)
    }

    /// Retrieve model for a given agent or tool name.
    /// If key is "planner" or "plan_mission", returns planner_model unless specifically overridden.
    pub fn get_model(&self, agent_name: &str) -> &str {
        if let Some(model) = self.agents.get(agent_name) {
            return model.as_str();
        }

        if agent_name.eq_ignore_ascii_case("planner")
            || agent_name.eq_ignore_ascii_case("plan_mission")
        {
            return self.planner_model.as_str();
        }

        self.default_model.as_str()
    }

    /// Retrieve the planner model
    pub fn get_planner_model(&self) -> &str {
        &self.planner_model
    }

    /// Retrieve the default agent model
    pub fn get_default_model(&self) -> &str {
        &self.default_model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_models() {
        let config = AgentModelConfig {
            default_model: DEFAULT_FACTORY_MODEL.to_string(),
            planner_model: DEFAULT_PLANNER_MODEL.to_string(),
            agents: HashMap::new(),
        };
        assert_eq!(config.get_planner_model(), "gpt-oss-120b");
        assert_eq!(config.get_model("planner"), "gpt-oss-120b");
        assert_eq!(config.get_model("plan_mission"), "gpt-oss-120b");
        assert_eq!(config.get_model("auditor"), "ollama/qwen2.5:7b");
        assert_eq!(config.get_model("security_review"), "ollama/qwen2.5:7b");
        assert_eq!(config.get_model("deep_research"), "ollama/qwen2.5:7b");
        assert_eq!(config.get_model("unknown_agent"), "ollama/qwen2.5:7b");
    }

    #[test]
    fn test_yaml_parsing() {
        let yaml = r#"
default_model: "ollama/qwen2.5:7b"
planner_model: "gpt-oss-120b"
agents:
  custom_specialist: "specialist-v1"
"#;
        let config: AgentModelConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.get_planner_model(), "gpt-oss-120b");
        assert_eq!(config.get_model("custom_specialist"), "specialist-v1");
        assert_eq!(config.get_model("auditor"), "ollama/qwen2.5:7b");
    }
}
