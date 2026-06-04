// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use anyhow::Error;
use zu_icons_util::module::generate_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/simple-icons/icons";

const REMAPPING_NAMES: &[&str] = &[
    "1001tracklists",
    "123",
    "1and1",
    "1dot1dot1dot1",
    "1panel",
    "1password",
    "2fas",
    "2k",
    "30secondsofcode",
    "365datascience",
    "3m",
    "42",
    "4chan",
    "4d",
    "500px",
    "7zip",
    "99designs",
    "9gag",
    "box",
    "element",
    "icon",
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
