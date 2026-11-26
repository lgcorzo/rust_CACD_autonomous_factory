use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2aSkillMetadata {
    pub allowed_roles: Vec<String>,
    pub max_hop: u8,
    pub requires_mtls: bool,
    pub inference_priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiFunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillToolDescriptor {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: OpenAiFunctionDef,
    pub x_darkgravity_a2a: A2aSkillMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilityManifest {
    pub schema_version: String,
    pub service: String,
    pub skills: Vec<SkillToolDescriptor>,
}

fn make_skill(
    name: &str,
    description: &str,
    required: &[&str],
    allowed_roles: &[&str],
    priority: &str,
) -> SkillToolDescriptor {
    let mut props = serde_json::Map::new();
    for req in required {
        props.insert(
            (*req).to_string(),
            json!({ "type": "string", "description": format!("Parameter {}", req) }),
        );
    }
    SkillToolDescriptor {
        tool_type: "function".to_string(),
        function: OpenAiFunctionDef {
            name: name.to_string(),
            description: description.to_string(),
            parameters: json!({
                "type": "object",
                "properties": props,
                "required": required,
                "additionalProperties": false
            }),
        },
        x_darkgravity_a2a: A2aSkillMetadata {
            allowed_roles: allowed_roles.iter().map(|s| (*s).to_string()).collect(),
            max_hop: 3,
            requires_mtls: true,
            inference_priority: priority.to_string(),
        },
    }
}

pub fn build_skills_manifest() -> AgentCapabilityManifest {
    let skills = vec![
        make_skill(
            "speckit.specify",
            "Create or update a feature specification",
            &["description"],
            &["rustant", "planner"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.clarify",
            "Identify and resolve underspecified areas in spec",
            &["spec_path"],
            &["rustant", "planner"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.plan",
            "Generate implementation plan from spec",
            &["spec_path"],
            &["rustant", "planner"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.tasks",
            "Generate dependency-ordered tasks.md",
            &["plan_path"],
            &["rustant", "planner"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.analyze",
            "Cross-artifact consistency analysis",
            &["feature_dir"],
            &["rustant", "reviewer"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.checklist",
            "Generate custom verification checklist",
            &["feature_dir"],
            &["rustant", "reviewer"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.constitution",
            "Create or update project constitution",
            &["principles"],
            &["rustant", "architect"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.implement",
            "Execute implementation plan tasks",
            &["tasks_path"],
            &["zeroclaw", "executor"],
            "Standard-Execution",
        ),
        make_skill(
            "speckit.taskstoissues",
            "Convert tasks.md into GitHub issues",
            &["tasks_path"],
            &["rustant", "poller"],
            "Background-Exploratory",
        ),
        make_skill(
            "factory.retrieve_context",
            "Query R2R GraphRAG knowledge base",
            &["query"],
            &["rustant", "zeroclaw", "aethelgard"],
            "Standard-Execution",
        ),
        make_skill(
            "factory.ingest_document",
            "Ingest artifact into R2R primary store",
            &["path", "collection"],
            &["rustant", "zeroclaw"],
            "Background-Exploratory",
        ),
        make_skill(
            "factory.security_review",
            "Run Aethelgard LLM-as-a-Judge SAST review",
            &["diff"],
            &["aethelgard", "rustant"],
            "Critical-Blocking",
        ),
        make_skill(
            "factory.detect_conflicts",
            "Detect semantic plan conflicts via Semantica",
            &["mission_id"],
            &["rustant", "semantica"],
            "Standard-Execution",
        ),
        make_skill(
            "factory.launch_sandbox_pod",
            "Launch isolated gVisor execution sandbox",
            &["task_id"],
            &["zeroclaw"],
            "Standard-Execution",
        ),
        make_skill(
            "factory.run_tdd_cycle",
            "Execute Red-Green-Refactor TDD loop",
            &["task_id", "test_cmd"],
            &["zeroclaw"],
            "Standard-Execution",
        ),
        make_skill(
            "factory.generate_openwiki",
            "Generate OKF / ISO-42010 architecture wiki",
            &["repo_path"],
            &["zeroclaw", "rustant"],
            "Background-Exploratory",
        ),
        make_skill(
            "factory.deliver_pr",
            "Create and verify Pull Request delivery",
            &["branch", "title"],
            &["rustant", "deliverer"],
            "Standard-Execution",
        ),
        make_skill(
            "factory.a2a_delegate",
            "Delegate scoped task to peer agent over Ziti mTLS",
            &["target", "skill_name"],
            &["rustant"],
            "Standard-Execution",
        ),
    ];
    AgentCapabilityManifest {
        schema_version: "1.0.0".to_string(),
        service: "factory-mcp-server".to_string(),
        skills,
    }
}

pub struct ContextSkill;

impl ContextSkill {
    pub fn prune_context(&self, raw_context: &str, max_chars: usize) -> String {
        if raw_context.len() <= max_chars {
            return raw_context.to_string();
        }

        // Simple pruning: take the first N characters but try to break at a newline
        // Safely find the character boundary
        let mut boundary = max_chars;
        while boundary > 0 && !raw_context.is_char_boundary(boundary) {
            boundary -= 1;
        }

        let pruned = &raw_context[..boundary];
        if let Some(last_newline) = pruned.rfind('\n') {
            pruned[..last_newline].to_string()
        } else {
            pruned.to_string()
        }
    }

    pub fn format_for_llm(&self, pruned_context: &str) -> Value {
        json!({
            "context_type": "vector_search",
            "content": pruned_context,
            "status": "pruned"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prune_context_no_pruning() {
        let skill = ContextSkill;
        let context = "short context";
        assert_eq!(skill.prune_context(context, 20), context);
    }

    #[test]
    fn test_prune_context_with_newline() {
        let skill = ContextSkill;
        let context = "line 1\nline 2\nline 3";
        // max_chars = 10 -> "line 1\nlin"
        // last newline is at index 6
        assert_eq!(skill.prune_context(context, 10), "line 1");
    }

    #[test]
    fn test_prune_context_without_newline() {
        let skill = ContextSkill;
        let context = "alongcontextwithoutnewlines";
        assert_eq!(skill.prune_context(context, 5), "along");
    }

    #[test]
    fn test_prune_context_empty() {
        let skill = ContextSkill;
        assert_eq!(skill.prune_context("", 10), "");
    }

    #[test]
    fn test_prune_context_max_zero() {
        let skill = ContextSkill;
        assert_eq!(skill.prune_context("anything", 0), "");
    }

    #[test]
    fn test_format_for_llm() {
        let skill = ContextSkill;
        let pruned = "pruned content";
        let result = skill.format_for_llm(pruned);
        assert_eq!(result["context_type"], "vector_search");
        assert_eq!(result["content"], pruned);
        assert_eq!(result["status"], "pruned");
    }

    #[test]
    fn test_prune_context_unicode_boundary() {
        let skill = ContextSkill;
        let context = "🦀🦀🦀"; // each 🦀 is 4 bytes
                                // Safe boundary
        assert_eq!(skill.prune_context(context, 4), "🦀");
    }

    #[test]
    fn test_prune_context_unicode_invalid_boundary() {
        let skill = ContextSkill;
        let context = "🦀🦀🦀";
        // 2 is not a char boundary for 🦀, should fallback to previous boundary (0)
        assert_eq!(skill.prune_context(context, 2), "");
    }
}
