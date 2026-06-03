// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct WindowMinimize {}

impl IconShape for WindowMinimize {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 416c0-17.7 14.3-32 32-32l448 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 448c-17.7 0-32-14.3-32-32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
