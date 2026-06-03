// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct SquareH {}

impl IconShape for SquareH {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM320 168l0 176c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-64-96 0 0 64c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-176c0-13.3 10.7-24 24-24s24 10.7 24 24l0 64 96 0 0-64c0-13.3 10.7-24 24-24s24 10.7 24 24z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
