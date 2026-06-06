// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use std::path::Path;

use anyhow::{Context, Error};

/// Read the `default` feature list from a `Cargo.toml` file.
///
/// Parses the given `Cargo.toml` and returns the list of features enabled by
/// default (i.e. the values in `[features].default`).
///
/// Returns an empty vector if the `[features]` table or the `default` key is
/// absent.
///
/// # Errors
///
/// Returns `Err` if the file cannot be read or contains invalid TOML.
pub fn get_default_features<P: AsRef<Path>>(cargo_toml_path: P) -> Result<Vec<String>, Error> {
    let content = std::fs::read_to_string(cargo_toml_path.as_ref())
        .with_context(|| format!("Failed to read {}", cargo_toml_path.as_ref().display()))?;
    let manifest: toml::Value = toml::from_str(&content).context("Failed to parse Cargo.toml")?;

    let default = manifest
        .get("features")
        .and_then(|f| f.get("default"))
        .and_then(|d| d.as_array());

    match default {
        Some(arr) => arr
            .iter()
            .map(|v| {
                v.as_str()
                    .map(String::from)
                    .ok_or_else(|| anyhow::anyhow!("Non-string value in default features"))
            })
            .collect(),
        None => Ok(Vec::new()),
    }
}

/// Extract icon struct names from a generated Rust source file.
///
/// Reads the file line by line and collects every identifier from lines
/// matching the pattern `pub struct ICON_NAME;`.
///
/// # Errors
///
/// Returns `Err` if the file cannot be read.
pub fn get_icon_list<P: AsRef<Path>>(rs_file: P) -> Result<Vec<String>, Error> {
    let content = std::fs::read_to_string(rs_file.as_ref())
        .with_context(|| format!("Failed to read {}", rs_file.as_ref().display()))?;

    let mut icons = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        // Match lines like: pub struct SomeIconName;
        if let Some(rest) = trimmed.strip_prefix("pub struct ") {
            if let Some(name) = rest.strip_suffix(';') {
                let name = name.trim();
                if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    icons.push(name.to_owned());
                }
            }
        }
    }

    Ok(icons)
}
