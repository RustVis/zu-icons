// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct BlackTie {}

impl IconShape for BlackTie {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 32l0 448 448 0 0-448-448 0zM316.5 357.2l-92.5 88.7-92.5-88.7 64.5-184-64.5-86.6 184.9 0-64.4 86.6 64.5 184z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
