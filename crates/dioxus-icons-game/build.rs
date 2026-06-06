// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use anyhow::Error;
use zu_icons_util::module::generate_variants_icons;
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

const VARIANT_LIST: &[&str] = &[
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
    generate_variants_icons(VARIANT_LIST, SVG_DIR, REMAPPING_NAMES)
}

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        rebuild_icons()?;
    }
    Ok(())
}
