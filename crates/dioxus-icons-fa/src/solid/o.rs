// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct O {}

impl IconShape for O {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M224 96a160 160 0 1 0 0 320 160 160 0 1 0 0-320zM448 256a224 224 0 1 1 -448 0 224 224 0 1 1 448 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
