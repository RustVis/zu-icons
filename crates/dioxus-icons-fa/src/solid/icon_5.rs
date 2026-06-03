// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Icon5 {}

impl IconShape for Icon5 {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 64C0 46.3 14.3 32 32 32l224 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-192 0 0 112 120 0c75.1 0 136 60.9 136 136S259.1 480 184 480L32 480c-17.7 0-32-14.3-32-32s14.3-32 32-32l152 0c39.8 0 72-32.2 72-72s-32.2-72-72-72L32 272c-17.7 0-32-14.3-32-32L0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
