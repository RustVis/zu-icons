// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use icon_util::{TEMPLATE_FILE, get_svg_inner, need_update};
use inflections::Inflect;

const SVG_DIR: &str = "../../icons/font-awesome";
const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

";

fn build_icons() -> Result<Vec<(String, String)>, io::Error> {
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
            println!("Ignore non svg file {path:?}");
            continue;
        }

        let stem = path.file_stem().unwrap();
        let stem_str = stem.to_str().unwrap();
        let data_name = &stem_str;
        let node_name = stem_str.to_pascal_case();
        let module_name = stem_str.to_snake_case();
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src");
        rs_filepath.push(&module_name);
        rs_filepath.set_extension("rs");

        let svg_content = fs::read_to_string(&path).unwrap();
        let markup = get_svg_inner(&svg_content).unwrap();
        let rs_content = TEMPLATE_FILE
            .replace("NODE_NAME", &node_name)
            .replace("DATA_NAME", data_name)
            .replace("MARKUP", markup);

        fs::write(rs_filepath, rs_content).unwrap();
        module_names.push((module_name, node_name));
    }

    module_names.sort();
    Ok(module_names)
}

fn rebuild_icons() -> Result<(), Box<dyn Error>> {
    println!("rebuild icons");
    let module_names = build_icons()?;

    let mut lib_file = File::create("src/lib.rs")?;
    lib_file.write_all(LIB_HEADER.as_bytes())?;
    for (module_name, node_name) in &module_names {
        let line = format!(
            r#" mod {module_name};
pub use {module_name}::{node_name};

"#
        );
        lib_file.write_all(line.as_bytes())?;
    }

    Ok(())
}

fn main() {
    // Check UPDATE_DIOXUS_ICONS=1 environment
    if need_update() || true {
        rebuild_icons().unwrap();
    }
}
