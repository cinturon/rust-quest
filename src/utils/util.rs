pub fn sanitize_string(string: &str) -> String {
    string
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}