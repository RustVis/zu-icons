// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct LitecoinSign {}

impl IconShape for LitecoinSign {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M128 64c0-17.7-14.3-32-32-32S64 46.3 64 64l0 157.9-38.6 11c-12.7 3.6-20.1 16.9-16.5 29.7s16.9 20.1 29.7 16.5L64 271.8 64 448c0 17.7 14.3 32 32 32l256 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-224 0 0-162.5 134.6-38.5c12.7-3.6 20.1-16.9 16.5-29.7s-16.9-20.1-29.7-16.5L128 203.6 128 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
