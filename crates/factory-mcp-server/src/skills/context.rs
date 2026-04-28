use serde_json::{json, Value};

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
