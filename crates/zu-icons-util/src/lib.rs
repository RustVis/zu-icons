// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

pub mod icon_list;
pub mod module;
pub mod svg;

use std::{fs, io};

pub const UPDATE_ZU_ICONS: &str = "UPDATE_ZU_ICONS";
pub const UPDATE_ICONS_PAGE: &str = "UPDATE_ICONS_PAGE";

/// Check whether icon crate shall be refreshed.
///
/// Set `UPDATE_ZU_ICONS=1` environment to force update all icons.
#[inline]
#[must_use]
pub fn need_update() -> bool {
    std::env::var_os(UPDATE_ZU_ICONS).is_some_and(|val| val != "0" && val != "false")
}

#[inline]
#[must_use]
pub fn need_update_icons_page() -> bool {
    std::env::var_os(UPDATE_ICONS_PAGE).is_some_and(|val| val != "0" && val != "false")
}

/// Remove all files in `src/` directory of crate
///
/// # Errors
///
/// Returns `Err` if the `src/` directory cannot be removed or recreated.
pub fn reset_crate_source() -> Result<(), io::Error> {
    fs::remove_dir_all("src")?;
    fs::create_dir("src")?;
    fs::write("src/lib.rs", "")
}
