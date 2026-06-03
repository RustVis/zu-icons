// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct MartiniGlass {}

impl IconShape for MartiniGlass {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M2.4 51.8C7.4 39.8 19.1 32 32 32l448 0c12.9 0 24.6 7.8 29.6 19.8s2.2 25.7-6.9 34.9L288 301.3 288 448 352 448c17.7 0 32 14.3 32 32s-14.3 32-32 32l-192 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l64 0 0-146.7-214.6-214.6C.2 77.5-2.5 63.7 2.4 51.8zM354.7 144l48-48-293.5 0 48 48 197.5 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
