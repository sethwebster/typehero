fn split_and_filter(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

// Usage:
// split_and_filter("a,,b,c", ",") -> ["a", "b", "c"]
