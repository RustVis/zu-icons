// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use anyhow::Error;
use zu_icons_util::module::generate_variants_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/RemixIcon/icons";
const REMAPPING_NAMES: &[&str] = &[
    "24-hours-line",
    "24-hours-fill",
    "4k-line",
    "4k-fill",
    "box",
    "try",
];

const VARIANT_LIST: &[&str] = &[
    "Arrows",
    "Buildings",
    "Business",
    "Communication",
    "Design",
    "Development",
    "Device",
    "Document",
    "Editor",
    "Finance",
    "Food",
    "Game & Sports",
    "Health & Medical",
    "Logos",
    "Map",
    "Media",
    "Others",
    "System",
    "User & Faces",
    "Weather",
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
