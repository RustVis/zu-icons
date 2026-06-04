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

/// Generate icon rs files for all variant directories.
///
/// # Errors
///
/// Returns `Err` if the directory cannot be read, an SVG file cannot be parsed,
/// or the output module file cannot be written.
pub fn generate_variants_icons(
    variant_list: &[&str],
    svg_dir: &str,
    remapping_names: &[&str],
) -> Result<(), Error> {
    for feature in variant_list {
        generate_variant_icons(svg_dir, feature, remapping_names)?;
    }

    let mut module_file = File::create("src/lib.rs")?;
    let line = r"// Auto Generated! DO NOT EDIT!

pub use dioxus_icon_component::{Icon, IconProps, IconShape};

    ";
    module_file.write_all(line.as_bytes())?;
    for feature in variant_list {
        let module_name = to_module_name(feature);
        let feature_line = r#"
#[cfg(feature = "{MODULE_NAME}")]
pub mod {MODULE_NAME};
"#
        .replace("{MODULE_NAME}", &module_name);
        module_file.write_all(feature_line.as_bytes())?;
    }

    Ok(())
}

/// Generate icon rs files for a variant directory.
///
/// Scans SVG files in `base_dir/{variant_dirname}`, parses them, and generates
/// a Dioxus component module file at `src/{variant_dirname}.rs`.
///
/// # Errors
///
/// Returns `Err` if the directory cannot be read, an SVG file cannot be parsed,
/// or the output module file cannot be written.
pub fn generate_variant_icons(
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
        let node_name = to_node_name(&stem_str);
        let module_name = to_module_name(&stem_str);

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
    let variant_module_name = to_module_name(variant_dirname);
    let mut module_file = File::create(format!("src/{variant_module_name}.rs"))?;
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

/// Generate icon rs source files from a flat directory of SVG files.
///
/// Scans SVG files in `base_dir`, parses them, and generates
/// the main Dioxus component module at `src/lib.rs`.
///
/// # Errors
///
/// Returns `Err` if the directory cannot be read, an SVG file cannot be parsed,
/// or the output module file cannot be written.
pub fn generate_icons(base_dir: &str, remapping_names: &[&str]) -> Result<(), Error> {
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
        let node_name = to_node_name(&stem_str);
        let module_name = to_module_name(&stem_str);

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

/// Convert a directory name to a valid Rust module name.
///
/// Strips `&` characters, collapses double spaces, and converts the result
/// to `snake_case`. This is used when processing icon variant directories
/// whose names may contain characters or spacing that are invalid in Rust
/// identifiers.
///
/// # Example
///
/// ```
/// # use zu_icons_util::module::to_module_name;
/// assert_eq!(to_module_name("Games & Sports"), "games_sports");
/// ```
#[must_use]
pub fn to_module_name(dir_name: &str) -> String {
    dir_name
        .replace('&', "")
        .replace("  ", " ")
        .replace("__", "_")
        .to_snake_case()
}

/// Convert a directory name to a valid Rust `PascalCase` node/component name.
///
/// Strips `&` characters, collapses double spaces, and converts the result
/// to `PascalCase`. This is used when generating a Dioxus component name
/// from an SVG filename or variant directory name.
///
/// # Example
///
/// ```
/// # use zu_icons_util::module::to_node_name;
/// assert_eq!(to_node_name("Games & Sports"), "GamesSports");
/// assert_eq!(to_node_name("fork_&_knife"), "ForkKnife");
/// ```
#[must_use]
pub fn to_node_name(dir_name: &str) -> String {
    dir_name
        .replace('&', "")
        .replace("  ", " ")
        .replace("__", "_")
        .to_pascal_case()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_module_name_games_sports() {
        assert_eq!(to_module_name("Games & Sports"), "games_sports");
    }

    #[test]
    fn test_to_node_name_games_sports() {
        assert_eq!(to_node_name("Games & Sports"), "GamesSports");
        assert_eq!(to_node_name("fork_&_knife"), "ForkKnife");
    }
}
