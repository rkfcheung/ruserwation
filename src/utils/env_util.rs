use std::env;

pub fn is_prod() -> bool {
    var_as_str("APP_ENV") == "prod"
}

pub fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn truncate_string(input: &str, max_len: usize) -> String {
    input.chars().take(max_len).collect()
}

pub fn var_as_bool_or(key: &str, default: bool) -> bool {
    env::var(key).map(|v| v == "true").unwrap_or(default)
}

pub fn var_as_int_or(key: &str, default: i32) -> i32 {
    var_as_str_or(key, &default.to_string())
        .parse()
        .unwrap_or(default)
}

pub fn var_as_str(key: &str) -> String {
    env::var(key).unwrap_or_default()
}

pub fn var_as_str_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(default.to_string())
}
