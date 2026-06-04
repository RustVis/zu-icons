// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::fs::File;
use std::io::Write;

use anyhow::Error;
use inflections::Inflect;
use zu_icons_util::module::build_variant_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/game-icons";
const REMAPPING_NAMES: &[&str] = &[
    "3d-meeple",
    "3d-hammer",
    "3d-glasses",
    "3d-stairs",
    "box",
    "try",
];

const FEATURE_LIST: &[&str] = &[
    "andymeneely",
    "aussiesim",
    "badges",
    "carl-olsen",
    "caro-asercion",
    "cathelineau",
    "catsu",
    "darkzaitzev",
    "delapouite",
    "faithtoken",
    "felbrigg",
    "generalace135",
    "guard13007",
    "heavenly-dog",
    "irongamer",
    "john-colburn",
    "john-redman",
    "kier-heyl",
    "lorc",
    "lord-berandas",
    "lucasms",
    "pepijn-poolman",
    "pierre-leducq",
    "priorblue",
    "quoting",
    "rihlsul",
    "sbed",
    "seregacthtuf",
    "skoll",
    "sparker",
    "spencerdub",
    "starseeker",
    "various-artists",
    "viscious-speed",
    "willdabeast",
    "zajkonur",
    "zeromancer",
];

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
        let feature_line = r#"
#[cfg(feature = "{FEATURE_NAME}")]
pub mod {MODULE_NAME};
"#
        .replace("{FEATURE_NAME}", feature)
        .replace("{MODULE_NAME}", &feature.to_snake_case());
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
