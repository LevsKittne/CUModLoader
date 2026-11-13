use std::collections::HashMap;
use std::path::Path;

pub const DEFAULT: &str = "\
[loader]
enabled = true
console = true
unity_log = true

[mods]
disabled =
";

pub struct Config {
    values: HashMap<String, String>,
}

impl Config {
    pub fn load(path: &Path) -> Self {
        if !path.exists() {
            let _ = std::fs::write(path, DEFAULT);
        }
        let text = std::fs::read_to_string(path).unwrap_or_else(|_| DEFAULT.to_owned());
        Self { values: parse(&text) }
    }

    pub fn flag(&self, key: &str, default: bool) -> bool {
        match self.values.get(key).map(|v| v.to_ascii_lowercase()) {
            Some(v) if v == "true" || v == "1" || v == "yes" => true,
            Some(v) if v == "false" || v == "0" || v == "no" => false,
            _ => default,
        }
    }
}

fn parse(text: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut section = String::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].trim().to_ascii_lowercase();
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            out.insert(format!("{section}.{}", k.trim().to_ascii_lowercase()), v.trim().to_owned());
        }
    }
    out
}
