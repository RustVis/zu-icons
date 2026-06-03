// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Error;
use inflections::Inflect;
use zu_icon_util::{generate_svg_component, need_update, parse_svg_content, reset_crate_source};

const SVG_DIR: &str = "../../icons/bootstrap/icons";
const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;
pub use dioxus_icon_component::{Icon, IconProps, IconShape};

";

fn map_filename(name: &str) -> String {
    let names = vec![
        "0-circle-fill",
        "0-circle",
        "0-square-fill",
        "0-square",
        "1-circle-fill",
        "1-circle",
        "1-square-fill",
        "1-square",
        "2-circle-fill",
        "2-circle",
        "2-square-fill",
        "2-square",
        "3-circle-fill",
        "3-circle",
        "3-square-fill",
        "3-square",
        "4-circle-fill",
        "4-circle",
        "4-square-fill",
        "4-square",
        "5-circle-fill",
        "5-circle",
        "5-square-fill",
        "5-square",
        "6-circle-fill",
        "6-circle",
        "6-square-fill",
        "6-square",
        "7-circle-fill",
        "7-circle",
        "7-square-fill",
        "7-square",
        "8-circle-fill",
        "8-circle",
        "8-square-fill",
        "8-square",
        "9-circle-fill",
        "9-circle",
        "9-square-fill",
        "9-square",
        "123",
        "box",
        "option",
        "try",
        "type",
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
    module_content: String,
}

fn build_icons() -> Result<(), Error> {
    let mut module_names = Vec::new();

    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);
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
            let module_content = generate_svg_component(&node_name, None, &svg_obj);
            module_names.push(ModuleInfo {
                module_name,
                node_name,
                module_content,
            });
        } else {
            eprintln!("Failed to parse svg file {}", path.display());
        }
    }

    module_names.sort();

    // Write to module file.
    let mut module_file = File::create("src/lib.rs")?;
    module_file.write_all(LIB_HEADER.as_bytes())?;
    for ModuleInfo {
        module_name: _module_name,
        node_name: _node_name,
        module_content,
    } in module_names
    {
        module_file.write_all(module_content.as_bytes())?;
    }

    Ok(())
}

fn rebuild_icons() -> Result<(), Error> {
    build_icons()?;

    Ok(())
}

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        rebuild_icons()?;
    }
    Ok(())
}
