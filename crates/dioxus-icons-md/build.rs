// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Error;
use zu_icons_util::module::{map_filename, to_module_name, to_node_name, ModuleInfo, MOD_HEADER};
use zu_icons_util::svg::{generate_svg_component, parse_svg_content};
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/material-design-icons/src";
const SVG_FILENAME: &str = "24px.svg";
const REMAPPING_NAMES: &[&str] = &[
    "10k",
    "10mp",
    "11mp",
    "123",
    "12mp",
    "13mp",
    "14mp",
    "15mp",
    "16mp",
    "17mp",
    "18_up_rating",
    "18mp",
    "19mp",
    "1k",
    "1k_plus",
    "1x_mobiledata",
    "20mp",
    "21mp",
    "22mp",
    "23mp",
    "24mp",
    "2k",
    "2k_plus",
    "3k",
    "3k_plus",
    "4k",
    "4k_plus",
    "2mp",
    "30fps",
    "30fps_select",
    "360",
    "3d_rotation",
    "3mp",
    "4mp",
    "5mp",
    "6mp",
    "7mp",
    "3p",
    "5g",
    "5k",
    "6k",
    "7k",
    "5k_plus",
    "60fps",
    "60fps_select",
    "6_ft_apart",
    "6k_plus",
    "7k_plus",
    "8k",
    "8k_plus",
    "8mp",
    "9k",
    "9k_plus",
    "9mp",
    "3g_mobiledata",
    "4g_mobiledata",
    "4g_plus_mobiledata",
    "box",
    "try",
];
const VARIANT_LIST: &[(&str, &str)] = &[
    ("materialicons", "filled"),
    ("materialiconsoutlined", "outlined"),
    ("materialiconsround", "round"),
    ("materialiconssharp", "sharp"),
    ("materialiconstwotone", "twotone"),
];

const CATEGORY_LIST: &[&str] = &[
    "action",
    "alert",
    "av",
    "communication",
    "content",
    "device",
    "editor",
    "file",
    "hardware",
    "home",
    "image",
    "maps",
    "navigation",
    "notification",
    "places",
    "social",
    "toggle",
];

// 0. walk through variant name
// 1. list all categories to get all icon names
// 2. list each icon name directory, to get available variant kind
// 3. save variant kind to list
fn rebuild_icons() -> Result<(), Error> {
    for (variant_dir, variant_name) in VARIANT_LIST {
        let mut module_names = Vec::new();
        for category in CATEGORY_LIST {
            let mut category_dir = PathBuf::from(SVG_DIR);
            category_dir.push(category);
            for entry in fs::read_dir(&category_dir)? {
                let entry = entry?;
                let mut path = entry.path();
                let Some(stem) = path.file_stem() else {
                    eprintln!("Ignore file without stem: {}", path.display());
                    continue;
                };
                let Some(stem_str) = stem.to_str() else {
                    eprintln!("Ignore file with non-utf-8 name: {}", path.display());
                    continue;
                };

                let stem_str = map_filename(stem_str, REMAPPING_NAMES);
                if !path.is_dir() {
                    continue;
                }
                // construct real svg path
                path.push(variant_dir);
                path.push(SVG_FILENAME);
                if !path.is_file() {
                    // not found, skipped
                    continue;
                }
                let svg_content = fs::read_to_string(&path)?;
                if let Some(svg_obj) = parse_svg_content(&svg_content) {
                    let node_name = to_node_name(&stem_str);
                    let module_name = to_module_name(&stem_str);
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
        }

        module_names.sort();

        // Generate module file
        let mut module_file = File::create(format!("src/{variant_name}.rs"))?;

        module_file.write_all(MOD_HEADER.as_bytes())?;
        for ModuleInfo {
            module_name: _module_name,
            node_name: _node_name,
            module_content,
        } in module_names
        {
            module_file.write_all(module_content.as_bytes())?;
        }
    }

    Ok(())
}

fn generate_lib_file() -> Result<(), Error> {
    let mut lib_file = File::create("src/lib.rs")?;
    let lib_header = r#"
// Auto Generated! DO NOT EDIT!

pub use dioxus_icon_component::{Icon, IconProps, IconShape};

#[cfg(feature = "filled")]
pub mod filled;

#[cfg(feature = "outlined")]
pub mod outlined;

#[cfg(feature = "round")]
pub mod round;

#[cfg(feature = "sharp")]
pub mod sharp;

#[cfg(feature = "twotone")]
pub mod twotone;
"#;

    lib_file.write_all(lib_header.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        generate_lib_file()?;
        rebuild_icons()?;
    }
    Ok(())
}
