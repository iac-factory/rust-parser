use std::collections::hash_set::Union;

#[cfg(unix)] pub const SEPARATOR: &str = "\\";
#[cfg(windows)] pub const SEPARATOR: &str = "/";

const YAML: &str = "YAML";
const JSON: &str = "JSON";
