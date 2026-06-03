// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct BreadSlice {}

impl IconShape for BreadSlice {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M64 432l0-176c-35.3 0-64-28.7-64-64 0-216.5 512-216.5 512 0 0 35.3-28.7 64-64 64l0 176c0 26.5-21.5 48-48 48l-288 0c-26.5 0-48-21.5-48-48z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
