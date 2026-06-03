// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Hexagon {}

impl IconShape for Hexagon {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M33.5 220.3c-12.7 22.2-12.7 49.4 0 71.5l96.2 168.1c12.8 22.4 36.7 36.2 62.5 36.2l191.6 0c25.8 0 49.7-13.8 62.5-36.2l96.2-168.1c12.7-22.2 12.7-49.4 0-71.5L446.3 52.2C433.5 29.8 409.6 16 383.8 16L192.2 16c-25.8 0-49.7 13.8-62.5 36.2L33.5 220.3z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
