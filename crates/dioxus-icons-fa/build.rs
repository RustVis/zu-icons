// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use icon_util::{TEMPLATE_FILE, get_svg_inner, need_update, reset_crate_source};
use inflections::Inflect;

const SVG_DIR: &str = "../../icons/font-awesome/svgs";
const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

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
        "type",
    ];
    if names.contains(&name) {
        return format!("icon-{name}");
    }
    name.to_string()
}

fn build_icons(folder: &str) -> Result<Vec<String>, io::Error> {
    let mut module_names = Vec::new();
    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);
    dir.push(folder);

    let svg_extension = OsStr::new("svg");

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("path: {path:?}");
        if !path.is_file() {
            continue;
        }
        if path.extension() != Some(svg_extension) {
            println!("Ignore non svg file {path:?}");
            continue;
        }

        let stem = path.file_stem().unwrap();
        let stem_str = stem.to_str().unwrap();
        let stem_str = map_filename(stem_str);
        // let data_name = &stem_str;
        let node_name = stem_str.to_pascal_case();
        let module_name = stem_str.to_snake_case();
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src");
        rs_filepath.push(folder);
        // Ignores EEXIST
        let _ret = fs::create_dir(&rs_filepath);
        rs_filepath.push(&module_name);
        rs_filepath.set_extension("rs");

        let svg_content = fs::read_to_string(&path).unwrap();
        let markup = get_svg_inner(&svg_content).unwrap();
        let rs_content = TEMPLATE_FILE
            .replace("ICON_NAME", &node_name)
            .replace("ICON_PATH", markup);

        fs::write(rs_filepath, rs_content).unwrap();
        module_names.push((module_name, node_name));
        break;
    }

    module_names.sort();

    // Write to module file.
    let mut module_file = File::create(format!("src/{}.rs", folder))?;
    let mut feature_names = Vec::new();
    module_file.write_all(LIB_HEADER.as_bytes())?;
    for (module_name, node_name) in &module_names {
        let feature_name = format!("{folder}_{module_name}");
        let line = format!(
            r#"
#[cfg(feature = "{feature_name}")]
mod {module_name};
#[cfg(feature = "{feature_name}")]
pub use {module_name}::{node_name};

"#
        );
        module_file.write_all(line.as_bytes())?;
        feature_names.push(feature_name);
    }

    Ok(feature_names)
}

fn rebuild_icons() -> Result<(), Box<dyn Error>> {
    println!("rebuild icons");
    let brand_features = build_icons("brands")?;
    let regular_features = build_icons("regular")?;
    let solid_features = build_icons("solid")?;

    let mut module_file = File::create("src/lib.rs")?;
    let line = r#"// Auto Generated! DO NOT EDIT!

pub mod brands;
pub mod regular;
pub mod solid;
    "#;
    module_file.write_all(line.as_bytes())?;

    // let mut cargo_file = File::open("Cargo.toml")?;
    // cargo_file.write_all(feature_lines.as_bytes())?;

    Ok(())
}

fn main() {
    // Check UPDATE_DIOXUS_ICONS=1 environment
    if need_update() {
        reset_crate_source().unwrap();
        rebuild_icons().unwrap();
    }
}
