// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Bridge {}

impl IconShape for Bridge {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l8 0 0 64-40 0 0 112c37.6 9.4 64 43.2 64 82l0 94c0 17.7 14.3 32 32 32l32 0c17.7 0 32-14.3 32-32l0-64c0-53 43-96 96-96s96 43 96 96l0 64c0 17.7 14.3 32 32 32l32 0c17.7 0 32-14.3 32-32l0-94c0-38.8 26.4-72.6 64-82l0-112-40 0 0-64 8 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 32zM424 96l0 64-80 0 0-64 80 0zM296 96l0 64-80 0 0-64 80 0zM88 96l80 0 0 64-80 0 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
