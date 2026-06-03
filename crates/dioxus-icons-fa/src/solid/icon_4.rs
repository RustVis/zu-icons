// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Icon4 {}

impl IconShape for Icon4 {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 288c0 35.3 28.7 64 64 64l192 0 0 96c0 17.7 14.3 32 32 32s32-14.3 32-32l0-96 32 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-32 0 0-224c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 224-192 0 0-224z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
