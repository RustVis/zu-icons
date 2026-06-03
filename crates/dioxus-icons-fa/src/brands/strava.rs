// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Strava {}

impl IconShape for Strava {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M158.4 0L7 292 96.2 292 158.4 175.9 220.1 292 308.6 292 158.4 0zM308.6 292l-43.9 88.2-44.6-88.2-67.6 0 112.2 220 111.5-220-67.6 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
