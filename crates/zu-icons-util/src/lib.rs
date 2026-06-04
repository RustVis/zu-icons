// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

pub mod module;
pub mod svg;

use std::{fs, io};

pub const UPDATE_KEY: &str = "UPDATE_ZU_ICONS";

/// Check whether icon crate shall be refreshed.
///
/// Set `UPDATE_ZU_ICONS=1` environment to force update all icons.
#[inline]
#[must_use]
pub fn need_update() -> bool {
    std::env::var_os(UPDATE_KEY).is_some_and(|val| val != "0" && val != "false")
}

/// Remove all files in `src/` directory of crate
pub fn reset_crate_source() -> Result<(), io::Error> {
    fs::remove_dir_all("src")?;
    fs::create_dir("src")?;
    fs::write("src/lib.rs", "")
}
