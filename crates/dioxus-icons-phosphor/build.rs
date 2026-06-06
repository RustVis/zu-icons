// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use anyhow::Error;
use zu_icons_util::module::generate_variants_icons;
use zu_icons_util::{need_update, reset_crate_source};

const SVG_DIR: &str = "../../icons/phosphor-icons/raw";
const REMAPPING_NAMES: &[&str] = &["box", "option", "try"];

const VARIANT_LIST: &[&str] = &["bold", "duotone", "fill", "light", "regular", "thin"];

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
