// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Bandcamp {}

impl IconShape for Bandcamp {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zm48.2 326.1l-181 0 84.7-156.1 181 0-84.7 156.1z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
