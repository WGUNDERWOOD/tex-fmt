use crate::regexes::*;

/// Record whether a line contains certain patterns to avoid recomputing
#[derive(Default)]
pub struct Pattern {
    /// Whether a begin environment pattern is present
    pub contains_env_begin: bool,
    /// Whether an end environment pattern is present
    pub contains_env_end: bool,
    /// Whether an item pattern is present
    pub contains_item: bool,
    /// Whether a splitting pattern is present
    pub contains_splitting: bool,
}

impl Pattern {
    /// Check if a string contains patterns
    pub fn new(s: &str) -> Self {
        let mut pattern = Self::default();

        // If splitting does not match, no patterns are present
        if RE_SPLITTING.is_match(s) {
            pattern.contains_env_begin = s.contains(ENV_BEGIN);
            pattern.contains_env_end = s.contains(ENV_END);
            pattern.contains_item = s.contains(ITEM);
            pattern.contains_splitting = true;
        }

        pattern
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    #[test]
    fn new_pattern() {
        let pattern =
            Pattern::new("\\begin{enumerate} \\end{enumerate} \\item ");
        assert!(pattern.contains_env_begin);
        assert!(pattern.contains_env_end);
        assert!(pattern.contains_item);
        assert!(pattern.contains_splitting);
    }
}
