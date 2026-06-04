// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::fs::File;
use std::io::Write;

use anyhow::Error;
use zu_icons_util::module::build_variant_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/heroicons/src/24";

const REMAPPING_NAMES: &[&str] = &[
    "0", "1", "11ty", "2", "3", "4", "42-group", "5", "500px", "6", "7", "8", "9", "box", "try",
];

fn rebuild_icons() -> Result<(), Error> {
    build_variant_icons(SVG_DIR, "outline", REMAPPING_NAMES)?;
    build_variant_icons(SVG_DIR, "solid", REMAPPING_NAMES)?;

    let mut module_file = File::create("src/lib.rs")?;
    let line = r#"// Auto Generated! DO NOT EDIT!

pub use dioxus_icon_component::{Icon, IconProps, IconShape};

#[cfg(feature = "outline")]
pub mod outline;
#[cfg(feature = "solid")]
pub mod solid;
    "#;
    module_file.write_all(line.as_bytes())?;

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
