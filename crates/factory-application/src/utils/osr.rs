pub fn calculate_osr(wiki_content: &str, r2r_text: &str) -> f32 {
    let distance = levenshtein_distance(wiki_content, r2r_text) as f32;
    let max_len = std::cmp::max(wiki_content.chars().count(), r2r_text.chars().count()) as f32;
    if max_len > 0.0 {
        distance / max_len
    } else {
        0.0
    }
}

#[allow(clippy::needless_range_loop)]
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();

    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];
    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for i in 0..len_a {
        for j in 0..len_b {
            if a_chars[i] == b_chars[j] {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] =
                    1 + std::cmp::min(dp[i][j], std::cmp::min(dp[i + 1][j], dp[i][j + 1]));
            }
        }
    }
    dp[len_a][len_b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_osr_calculation() {
        // Identical documents should yield 0.0
        let doc_a = "This is a documentation file about the system architecture.";
        let doc_b = "This is a documentation file about the system architecture.";
        assert_eq!(calculate_osr(doc_a, doc_b), 0.0);

        // Heavily divergent documents should yield > 0.05
        let doc_divergent = "This is something completely different about frogs.";
        let osr_divergent = calculate_osr(doc_a, doc_divergent);
        assert!(
            osr_divergent > 0.05,
            "Expected heavily divergent docs to have high OSR, got {}",
            osr_divergent
        );

        // Minor typos should yield a small positive OSR
        let doc_minor = "This is a documntation file about the system architecture.";
        let osr_minor = calculate_osr(doc_a, doc_minor);
        assert!(
            osr_minor > 0.0 && osr_minor < 0.05,
            "Expected minor typo OSR to be between 0 and 0.05, got {}",
            osr_minor
        );

        // Empty strings
        assert_eq!(calculate_osr("", ""), 0.0);
    }
}
