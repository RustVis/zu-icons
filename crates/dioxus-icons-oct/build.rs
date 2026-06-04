// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use anyhow::Error;
use zu_icons_util::module::build_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/octicons/icons";

const REMAPPING_NAMES: &[&str] = &["123", "box", "option", "try", "type"];

fn rebuild_icons() -> Result<(), Error> {
    build_icons(SVG_DIR, REMAPPING_NAMES)
}

fn main() -> Result<(), Error> {
    // Check UPDATE_ZU_ICONS=1 environment
    if need_update() {
        reset_crate_source()?;
        rebuild_icons()?;
    }
    Ok(())
}
