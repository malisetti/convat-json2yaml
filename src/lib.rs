#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use serde_json::Value;

/// Convert JSON-lines input to YAML with one document per input line on stdout.
pub fn convert(input: &str) -> Result<String> {
    let mut out = String::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(line)
            .with_context(|| format!("invalid JSON on line: {line}"))?;
        let yaml = serde_yaml::to_string(&value).context("failed to serialize value as YAML")?;
        let doc = yaml.trim_end();
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(doc);
    }
    if !out.is_empty() {
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_single_object() {
        let out = convert(r#"{"x":1}"#).unwrap();
        assert_eq!(out, "x: 1\n");
    }

    #[test]
    fn convert_multiple_lines() {
        let out = convert("{\"a\":1}\n{\"b\":2}\n").unwrap();
        assert_eq!(out, "a: 1\nb: 2\n");
    }
}
