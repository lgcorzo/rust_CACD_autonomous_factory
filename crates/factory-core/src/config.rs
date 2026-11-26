use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const DEFAULT_FACTORY_MODEL: &str = "ollama/qwen2.5:7b";
pub const DEFAULT_PLANNER_MODEL: &str = "gemini-3.8-flash-low";

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
    /// If key is "planner", "plan_mission", or "zeroclaw", returns planner_model unless specifically overridden.
    pub fn get_model(&self, agent_name: &str) -> &str {
        if let Some(model) = self.agents.get(agent_name) {
            return model.as_str();
        }

        if agent_name.eq_ignore_ascii_case("planner")
            || agent_name.eq_ignore_ascii_case("plan_mission")
            || agent_name.eq_ignore_ascii_case("zeroclaw")
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

pub const DEFAULT_SWARM_MAX_CONCURRENT_LANES: usize = 4;
pub const DEFAULT_SWARM_VELOCITY_WINDOW_SECS: u64 = 60;
pub const DEFAULT_SWARM_VELOCITY_MAX_RETRIES: u32 = 6;
pub const DEFAULT_A2A_MAX_HOPS: u8 = 3;
pub const DEFAULT_SANDBOX_PROFILE: &str = "micro-sandbox";

/// Runtime configuration for parallel Swarm Mode orchestration and circuit breakers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwarmConfig {
    pub max_concurrent_lanes: usize,
    pub velocity_window_secs: u64,
    pub velocity_max_retries: u32,
    pub a2a_max_hops: u8,
    pub default_sandbox_profile: String,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            max_concurrent_lanes: DEFAULT_SWARM_MAX_CONCURRENT_LANES,
            velocity_window_secs: DEFAULT_SWARM_VELOCITY_WINDOW_SECS,
            velocity_max_retries: DEFAULT_SWARM_VELOCITY_MAX_RETRIES,
            a2a_max_hops: DEFAULT_A2A_MAX_HOPS,
            default_sandbox_profile: DEFAULT_SANDBOX_PROFILE.to_string(),
        }
    }
}

impl SwarmConfig {
    /// Loads `SwarmConfig` from process environment variables, falling back to safe defaults.
    pub fn from_env() -> Self {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Loads `SwarmConfig` using a custom key lookup closure for deterministic testing.
    pub fn from_lookup<F>(lookup: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let defaults = Self::default();

        let max_concurrent_lanes = lookup("SWARM_MAX_CONCURRENT_LANES")
            .and_then(|v| v.trim().parse::<usize>().ok())
            .filter(|&v| v >= 1)
            .unwrap_or(defaults.max_concurrent_lanes);

        let velocity_window_secs = lookup("SWARM_VELOCITY_WINDOW_SECS")
            .and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|&v| v >= 1)
            .unwrap_or(defaults.velocity_window_secs);

        let velocity_max_retries = lookup("SWARM_VELOCITY_MAX_RETRIES")
            .and_then(|v| v.trim().parse::<u32>().ok())
            .filter(|&v| v >= 1)
            .unwrap_or(defaults.velocity_max_retries);

        let a2a_max_hops = lookup("SWARM_A2A_MAX_HOPS")
            .and_then(|v| v.trim().parse::<u8>().ok())
            .filter(|&v| v >= 1)
            .unwrap_or(defaults.a2a_max_hops);

        let default_sandbox_profile = lookup("SWARM_DEFAULT_SANDBOX_PROFILE")
            .map(|v| v.trim().to_ascii_lowercase())
            .filter(|v| v == "micro-sandbox" || v == "compilation-sandbox")
            .unwrap_or(defaults.default_sandbox_profile);

        Self {
            max_concurrent_lanes,
            velocity_window_secs,
            velocity_max_retries,
            a2a_max_hops,
            default_sandbox_profile,
        }
    }
}

/// Header name used to propagate semantic inference priority to LiteLLM.
pub const INFERENCE_PRIORITY_HEADER: &str = "X-Inference-Priority";

/// Semantic inference scheduling priority tier (`FR-011`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InferencePriorityClass {
    #[serde(rename = "Critical-Blocking")]
    CriticalBlocking,
    #[default]
    #[serde(rename = "Standard-Execution")]
    StandardExecution,
    #[serde(rename = "Background-Exploratory")]
    BackgroundExploratory,
}

impl InferencePriorityClass {
    /// Returns the wire-format header value expected by LiteLLM priority callbacks.
    pub fn as_header_value(&self) -> &'static str {
        match self {
            Self::CriticalBlocking => "Critical-Blocking",
            Self::StandardExecution => "Standard-Execution",
            Self::BackgroundExploratory => "Background-Exploratory",
        }
    }

    /// Returns the Redis priority weight (`100` / `50` / `10`).
    pub fn priority_weight(&self) -> u32 {
        match self {
            Self::CriticalBlocking => 100,
            Self::StandardExecution => 50,
            Self::BackgroundExploratory => 10,
        }
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
        assert_eq!(config.get_planner_model(), "gemini-3.8-flash-low");
        assert_eq!(config.get_model("planner"), "gemini-3.8-flash-low");
        assert_eq!(config.get_model("plan_mission"), "gemini-3.8-flash-low");
        assert_eq!(config.get_model("zeroclaw"), "gemini-3.8-flash-low");
        assert_eq!(config.get_model("auditor"), "ollama/qwen2.5:7b");
        assert_eq!(config.get_model("security_review"), "ollama/qwen2.5:7b");
        assert_eq!(config.get_model("deep_research"), "ollama/qwen2.5:7b");
        assert_eq!(config.get_model("unknown_agent"), "ollama/qwen2.5:7b");
    }

    #[test]
    fn test_yaml_parsing() {
        let yaml = r#"
default_model: "ollama/qwen2.5:7b"
planner_model: "gemini-3.8-flash-low"
agents:
  custom_specialist: "specialist-v1"
"#;
        let config: AgentModelConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.get_planner_model(), "gemini-3.8-flash-low");
        assert_eq!(config.get_model("custom_specialist"), "specialist-v1");
        assert_eq!(config.get_model("auditor"), "ollama/qwen2.5:7b");
    }

    #[test]
    fn test_swarm_config_defaults_and_env_overrides() {
        let defaults = SwarmConfig::default();
        assert_eq!(defaults.max_concurrent_lanes, 4);
        assert_eq!(defaults.velocity_window_secs, 60);
        assert_eq!(defaults.velocity_max_retries, 6);
        assert_eq!(defaults.a2a_max_hops, 3);
        assert_eq!(defaults.default_sandbox_profile, "micro-sandbox");

        let custom = SwarmConfig::from_lookup(|key| match key {
            "SWARM_MAX_CONCURRENT_LANES" => Some("8".to_string()),
            "SWARM_VELOCITY_WINDOW_SECS" => Some("90".to_string()),
            "SWARM_VELOCITY_MAX_RETRIES" => Some("10".to_string()),
            "SWARM_A2A_MAX_HOPS" => Some("2".to_string()),
            "SWARM_DEFAULT_SANDBOX_PROFILE" => Some("compilation-sandbox".to_string()),
            _ => None,
        });
        assert_eq!(custom.max_concurrent_lanes, 8);
        assert_eq!(custom.velocity_window_secs, 90);
        assert_eq!(custom.velocity_max_retries, 10);
        assert_eq!(custom.a2a_max_hops, 2);
        assert_eq!(custom.default_sandbox_profile, "compilation-sandbox");

        // Invalid or zero env values fall back safely
        let clamped = SwarmConfig::from_lookup(|key| match key {
            "SWARM_MAX_CONCURRENT_LANES" => Some("0".to_string()),
            "SWARM_DEFAULT_SANDBOX_PROFILE" => Some("privileged-host".to_string()),
            _ => None,
        });
        assert_eq!(clamped.max_concurrent_lanes, 4);
        assert_eq!(clamped.default_sandbox_profile, "micro-sandbox");
    }

    #[test]
    fn test_inference_priority_class_headers_and_weights() {
        assert_eq!(
            InferencePriorityClass::CriticalBlocking.as_header_value(),
            "Critical-Blocking"
        );
        assert_eq!(
            InferencePriorityClass::StandardExecution.as_header_value(),
            "Standard-Execution"
        );
        assert_eq!(
            InferencePriorityClass::BackgroundExploratory.as_header_value(),
            "Background-Exploratory"
        );
        assert!(
            InferencePriorityClass::CriticalBlocking.priority_weight()
                > InferencePriorityClass::StandardExecution.priority_weight()
        );
        assert!(
            InferencePriorityClass::StandardExecution.priority_weight()
                > InferencePriorityClass::BackgroundExploratory.priority_weight()
        );
    }
}
