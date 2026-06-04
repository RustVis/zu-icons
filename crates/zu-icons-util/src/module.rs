use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Error;
use inflections::Inflect;

use crate::svg::{generate_svg_component, parse_svg_content};

const MOD_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

use crate::IconShape;
";

const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;
pub use dioxus_icon_component::{Icon, IconProps, IconShape};

";

fn map_filename(name: &str, remapping_names: &[&str]) -> String {
    if remapping_names.contains(&name) {
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

/// Build icon source files for a variant directory.
///
/// Scans SVG files in `base_dir/{variant_dirname}`, parses them, and generates
/// a Dioxus component module file at `src/{variant_dirname}.rs`.
///
/// # Errors
///
/// Returns `Err` if the directory cannot be read, an SVG file cannot be parsed,
/// or the output module file cannot be written.
pub fn build_variant_icons(
    base_dir: &str,
    variant_dirname: &str,
    remapping_names: &[&str],
) -> Result<(), Error> {
    let mut module_names = Vec::new();

    let mut dir = PathBuf::new();
    dir.push(base_dir);
    dir.push(variant_dirname);
    let svg_extension = OsStr::new("svg");

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension() != Some(svg_extension) {
            eprintln!("Ignore non svg file {}", path.display());
            continue;
        }

        let Some(stem) = path.file_stem() else {
            eprintln!("Ignore file without stem: {}", path.display());
            continue;
        };
        let Some(stem_str) = stem.to_str() else {
            eprintln!("Ignore file with non-utf-8 name: {}", path.display());
            continue;
        };

        let stem_str = map_filename(stem_str, remapping_names);
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
    let mut module_file = File::create(format!("src/{variant_dirname}.rs"))?;
    module_file.write_all(MOD_HEADER.as_bytes())?;
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

/// Build icon source files from a flat directory of SVG files.
///
/// Scans SVG files in `base_dir`, parses them, and generates
/// the main Dioxus component module at `src/lib.rs`.
///
/// # Errors
///
/// Returns `Err` if the directory cannot be read, an SVG file cannot be parsed,
/// or the output module file cannot be written.
pub fn build_icons(base_dir: &str, remapping_names: &[&str]) -> Result<(), Error> {
    let mut module_names = Vec::new();

    let dir = PathBuf::from(base_dir);
    let svg_extension = OsStr::new("svg");

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension() != Some(svg_extension) {
            eprintln!("Ignore non svg file {}", path.display());
            continue;
        }

        let Some(stem) = path.file_stem() else {
            eprintln!("Ignore file without stem: {}", path.display());
            continue;
        };
        let Some(stem_str) = stem.to_str() else {
            eprintln!("Ignore file with non-utf-8 name: {}", path.display());
            continue;
        };

        let stem_str = map_filename(stem_str, remapping_names);
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
