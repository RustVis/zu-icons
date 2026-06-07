// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use std::{
    collections::BTreeMap,
    env,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Error};
use inflections::Inflect;

const ICON_TEMPLATE_STR: &str = r#"
div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: {MODULE_NAME}::{ICON_NAME},
    }
    span { "{ICON_NAME}" }
}
"#;
const ICON_CONTAINER_TEMPLATE: &str = r#"
use dioxus::prelude::*;
use {MODULE_NAME}::Icon;

#[component]
pub fn {CONTAINER_NAME}() -> Element {
  rsx!(
    div {
        display: "flex",
        flex_direction: "row",
        gap: "24px",
        flex_wrap: "wrap",
            {ICON_COMPONENTS}
        }
  )
}
"#;

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
    println!("get_icon_list() path: {}", rs_file.as_ref().display());
    let content = std::fs::read_to_string(rs_file.as_ref())?;
    println!("content: {}", content.len());
    assert!(!content.is_empty());

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

pub fn get_default_icon_crate_info<P: AsRef<Path>>(crate_path: P) -> Result<Vec<String>, Error> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let mut crate_abspath = PathBuf::from(manifest_dir);
    crate_abspath.push(&crate_path);
    crate_abspath.push("src");
    crate_abspath.push("lib.rs");
    let icon_list = get_icon_list(&crate_abspath)?;
    Ok(icon_list)
}

pub fn get_variant_icon_crate_info<P: AsRef<Path>>(
    crate_path: P,
) -> Result<BTreeMap<String, Vec<String>>, Error> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let mut crate_abspath = PathBuf::from(manifest_dir);
    crate_abspath.push(&crate_path);
    let cargo_file = crate_abspath.join("Cargo.toml");
    let default_features = get_default_features(&cargo_file)?;
    crate_abspath.push("src");
    crate_abspath.push("lib.rs");

    let mut map = BTreeMap::new();
    for feature_name in default_features {
        let rs_file = crate_abspath.with_file_name(format!("{feature_name}.rs"));
        let icon_list = get_icon_list(&rs_file)?;
        map.insert(feature_name, icon_list);
    }

    Ok(map)
}

pub fn generate_icon_component(module_name: &str, icon_name: &str) -> String {
    ICON_TEMPLATE_STR
        .replace("{MODULE_NAME}", module_name)
        .replace("{ICON_NAME}", icon_name)
}

pub fn generate_icon_components_container<P: AsRef<Path>>(
    module_name: &str,
    rs_file: P,
    component_name: &str,
    icon_components: &[String],
) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(rs_file)?;
    let content = ICON_CONTAINER_TEMPLATE
        .replace("{MODULE_NAME}", module_name)
        .replace("{CONTAINER_NAME}", component_name)
        .replace("{ICON_COMPONENTS}", &icon_components.join(""));
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn generate_default_icon_page(
    crate_path: &str,
    output_rs_file: &str,
    module_name: &str,
) -> Result<(), Error> {
    let icons = get_default_icon_crate_info(crate_path)?;
    assert!(!icons.is_empty());
    let mut icon_components = Vec::new();
    for icon_name in &icons {
        icon_components.push(generate_icon_component(module_name, icon_name));
    }
    generate_icon_components_container(module_name, output_rs_file, "IconsPage", &icon_components)?;

    Ok(())
}

pub fn generate_variant_icon_pages(crate_path: &str, module_name: &str) -> Result<(), Error> {
    let all_icons = get_variant_icon_crate_info(crate_path)?;
    assert!(!all_icons.is_empty());
    for (feature_name, icons) in &all_icons {
        let mut icon_components = Vec::new();
        let module_feature_name = format!("{module_name}::{feature_name}");
        for icon_name in icons {
            icon_components.push(generate_icon_component(&module_feature_name, icon_name));
        }
        generate_icon_components_container(
            module_name,
            &format!("src/{feature_name}_page.rs"),
            &format!("{}Page", feature_name.to_pascal_case()),
            &icon_components,
        )?;
    }

    Ok(())
}
