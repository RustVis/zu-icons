// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::fs::File;
use std::io::Write;

use anyhow::Error;
use zu_icons_util::module::{build_variant_icons, to_module_name};
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/phosphor-icons/raw";
const REMAPPING_NAMES: &[&str] = &["box", "option", "try"];

const FEATURE_LIST: &[&str] = &["bold", "duotone", "fill", "light", "regular", "thin"];

fn rebuild_icons() -> Result<(), Error> {
    for feature in FEATURE_LIST {
        build_variant_icons(SVG_DIR, feature, REMAPPING_NAMES)?;
    }

    let mut module_file = File::create("src/lib.rs")?;
    let line = r#"// Auto Generated! DO NOT EDIT!

pub use dioxus_icon_component::{Icon, IconProps, IconShape};

    "#;
    module_file.write_all(line.as_bytes())?;
    for feature in FEATURE_LIST {
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

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        rebuild_icons()?;
    }
    Ok(())
}
