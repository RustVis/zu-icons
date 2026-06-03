// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct U {}

impl IconShape for U {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32c17.7 0 32 14.3 32 32l0 224c0 70.7 57.3 128 128 128s128-57.3 128-128l0-224c0-17.7 14.3-32 32-32s32 14.3 32 32l0 224c0 106-86 192-192 192S0 394 0 288L0 64C0 46.3 14.3 32 32 32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
