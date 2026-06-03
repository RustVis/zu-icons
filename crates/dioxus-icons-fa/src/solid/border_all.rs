// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct BorderAll {}

impl IconShape for BorderAll {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M384 96l0 128-128 0 0-128 128 0zm0 192l0 128-128 0 0-128 128 0zM192 224l-128 0 0-128 128 0 0 128zM64 288l128 0 0 128-128 0 0-128zM64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
