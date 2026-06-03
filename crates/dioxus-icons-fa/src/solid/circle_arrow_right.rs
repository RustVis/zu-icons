// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct CircleArrowRight {}

impl IconShape for CircleArrowRight {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zm41-159c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l39-39-150.1 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l150.1 0-39-39c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l80 80c9.4 9.4 9.4 24.6 0 33.9l-80 80z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
