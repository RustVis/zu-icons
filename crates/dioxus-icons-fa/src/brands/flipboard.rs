// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Flipboard {}

impl IconShape for Flipboard {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 32l0 448 448 0 0-448-448 0zM358.4 211.2l-89.6 0 0 89.6-89.6 0 0 89.6-89.6 0 0-268.8 268.8 0 0 89.6z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
