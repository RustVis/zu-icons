// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use anyhow::Error;
use zu_icons_util::module::generate_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/bootstrap/icons";

const REMAPPING_NAMES: &[&str] = &[
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

fn rebuild_icons() -> Result<(), Error> {
    generate_icons(SVG_DIR, REMAPPING_NAMES)
}

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        rebuild_icons()?;
    }
    Ok(())
}
