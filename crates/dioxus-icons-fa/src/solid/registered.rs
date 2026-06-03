// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Registered {}

impl IconShape for Registered {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zM200 144l80 0c39.8 0 72 32.2 72 72 0 28.9-17 53.8-41.6 65.3l30.2 50.3c6.8 11.4 3.1 26.1-8.2 32.9s-26.1 3.1-32.9-8.2l-41-68.3-34.4 0 0 56c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-176c0-13.3 10.7-24 24-24zm72 96l8 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-56 0 0 48 48 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
