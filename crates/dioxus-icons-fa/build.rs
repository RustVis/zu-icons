// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Error;
use icon_util::{generate_svg_component, need_update, parse_svg_content, reset_crate_source};
use inflections::Inflect;

const SVG_DIR: &str = "../../icons/font-awesome/svgs";
const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

use crate::IconShape;
";

fn map_filename(name: &str) -> String {
    let names = vec![
        "0", "1", "11ty", "2", "3", "4", "42-group", "5", "500px", "6", "7", "8", "9", "box", "try",
    ];
    if names.contains(&name) {
        return format!("icon-{name}");
    }
    name.to_string()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ModuleInfo {
    module_name: String,
    node_name: String,
    feature_name: String,
    module_content: String,
}

fn build_icons(folder: &str) -> Result<Vec<String>, Error> {
    let mut module_names = Vec::new();

    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);
    dir.push(folder);
    let svg_extension = OsStr::new("svg");

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension() != Some(svg_extension) {
            eprintln!("Ignore non svg file {path:?}");
            continue;
        }

        let stem = path.file_stem().unwrap();
        let stem_str = stem.to_str().unwrap();

        let stem_str = map_filename(stem_str);
        // let data_name = &stem_str;
        let node_name = stem_str.to_pascal_case();
        let module_name = stem_str.to_snake_case();

        let svg_content = fs::read_to_string(&path)?;
        if let Some(svg_obj) = parse_svg_content(&svg_content) {
            let feature_name = format!("{folder}_{module_name}");
            let module_content = generate_svg_component(&feature_name, &node_name, None, &svg_obj);
            module_names.push(ModuleInfo {
                module_name,
                node_name,
                module_content,
                feature_name,
            });
        } else {
            eprintln!("Failed to parse svg file {}", path.display());
        }
    }

    module_names.sort();

    // Write to module file.
    let mut module_file = File::create(format!("src/{}.rs", folder))?;
    module_file.write_all(LIB_HEADER.as_bytes())?;
    let mut feature_names = Vec::new();
    for ModuleInfo {
        module_name: _module_name,
        node_name: _node_name,
        feature_name,
        module_content,
    } in module_names
    {
        module_file.write_all(module_content.as_bytes())?;
        feature_names.push(feature_name);
    }

    Ok(feature_names)
}

fn rebuild_icons() -> Result<(), Error> {
    let brands_features = build_icons("brands")?;
    let regular_features = build_icons("regular")?;
    let solid_features = build_icons("solid")?;

    let mut module_file = File::create("src/lib.rs")?;
    let line = r#"// Auto Generated! DO NOT EDIT!

pub use dioxus_icon_component::{Icon, IconProps, IconShape};

pub mod brands;
pub mod regular;
pub mod solid;
    "#;
    module_file.write_all(line.as_bytes())?;

    let mut cargo_file = File::create("feature.list")?;
    let (brands_group_features, brands_separated_features) =
        to_feature_string("brands", &brands_features);
    let (regular_group_features, regular_separated_features) =
        to_feature_string("regular", &regular_features);
    let (solid_group_features, solid_separated_features) =
        to_feature_string("solid", &solid_features);
    cargo_file.write_all(brands_group_features.as_bytes())?;
    cargo_file.write_all(regular_group_features.as_bytes())?;
    cargo_file.write_all(solid_group_features.as_bytes())?;
    cargo_file.write_all(brands_separated_features.as_bytes())?;
    cargo_file.write_all(regular_separated_features.as_bytes())?;
    cargo_file.write_all(solid_separated_features.as_bytes())?;

    Ok(())
}

fn to_feature_string(group_name: &str, feature_list: &[String]) -> (String, String) {
    let FEATURE_TEMPLATE = r#"{GROUP_NAME} = [
{FEATURE_LIST}
]
"#;
    let group_features = FEATURE_TEMPLATE
        .replace("{GROUP_NAME}", group_name)
        .replace(
            "{FEATURE_LIST}",
            &feature_list
                .iter()
                .map(|name| format!("  \"{name}\",\n"))
                .collect::<Vec<_>>()
                .join(""),
        );

    let separted_features = feature_list
        .iter()
        .map(|name| format!("{name} = []\n"))
        .collect::<Vec<_>>()
        .join("");
    (group_features, separted_features)
}

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        rebuild_icons()?;
    }
    Ok(())
}
