pub mod image;

pub fn shorten_id(id: &str) -> String {
    if id.is_empty() {
        return String::new();
    }
    id.split(':')
        .last()
        .unwrap_or(id)
        .chars()
        .take(12)
        .collect::<String>()
}

pub fn shorten_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length - 3 {
        return s.to_string();
    }
    format!("{}...", &s[..max_length - 3])
}
