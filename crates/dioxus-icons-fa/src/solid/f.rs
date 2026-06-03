// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct F {}

impl IconShape for F {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32C14.3 32 0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160 160 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-160 0 0-128 224 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
