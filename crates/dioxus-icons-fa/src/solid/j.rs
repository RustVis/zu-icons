// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct J {}

impl IconShape for J {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M288 32c17.7 0 32 14.3 32 32l0 256c0 88.4-71.6 160-160 160S0 408.4 0 320l0-32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 32c0 53 43 96 96 96s96-43 96-96l0-256c0-17.7 14.3-32 32-32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
