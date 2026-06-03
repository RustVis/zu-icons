// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct DiceSix {}

impl IconShape for DiceSix {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 96C0 60.7 28.7 32 64 32l320 0c35.3 0 64 28.7 64 64l0 320c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64L0 96zm160 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM128 288a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm32 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM320 192a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm32 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM320 384a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
