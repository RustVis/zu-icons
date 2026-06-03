// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Unsplash {}

impl IconShape for Unsplash {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M448 230.2l0 249.8-448 0 0-249.8 141.1 0 0 124.9 165.7 0 0-124.9 141.1 0zM306.9 32l-165.7 0 0 124.9 165.7 0 0-124.9z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
