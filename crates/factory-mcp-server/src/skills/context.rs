use serde_json::{json, Value};

pub struct ContextSkill;

impl ContextSkill {
    pub fn prune_context(&self, raw_context: &str, max_chars: usize) -> String {
        if raw_context.len() <= max_chars {
            return raw_context.to_string();
        }

        // Simple pruning: take the first N characters but try to break at a newline
        let pruned = &raw_context[..max_chars];
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
