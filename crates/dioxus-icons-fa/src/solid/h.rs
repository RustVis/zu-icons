// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct H {}

impl IconShape for H {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M320 288l0 160c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 160-256 0 0-160c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160 256 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
