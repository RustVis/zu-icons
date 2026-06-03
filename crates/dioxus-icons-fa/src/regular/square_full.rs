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
        d: "M448 48c8.8 0 16 7.2 16 16l0 384c0 8.8-7.2 16-16 16L64 464c-8.8 0-16-7.2-16-16L48 64c0-8.8 7.2-16 16-16l384 0zM64 0C28.7 0 0 28.7 0 64L0 448c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-384c0-35.3-28.7-64-64-64L64 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
