// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct LongArrowUp {}

impl IconShape for LongArrowUp {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M182.6-22.6c-12.5-12.5-32.8-12.5-45.3 0l-128 128c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L128 77.3 128 512c0 17.7 14.3 32 32 32s32-14.3 32-32l0-434.7 73.4 73.4c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3l-128-128z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
