// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct SquareFull {}

impl IconShape for SquareFull {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 64C0 28.7 28.7 0 64 0L448 0c35.3 0 64 28.7 64 64l0 384c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
