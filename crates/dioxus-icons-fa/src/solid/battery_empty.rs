// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct BatteryEmpty {}

impl IconShape for BatteryEmpty {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M112 160c-8.8 0-16 7.2-16 16l0 224c0 8.8 7.2 16 16 16l416 0c8.8 0 16-7.2 16-16l0-224c0-8.8-7.2-16-16-16l-416 0zM32 176c0-44.2 35.8-80 80-80l416 0c44.2 0 80 35.8 80 80l0 48c17.7 0 32 14.3 32 32l0 64c0 17.7-14.3 32-32 32l0 48c0 44.2-35.8 80-80 80l-416 0c-44.2 0-80-35.8-80-80l0-224z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 640 512");

}
