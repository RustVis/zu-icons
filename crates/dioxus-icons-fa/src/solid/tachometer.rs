// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Tachometer {}

impl IconShape for Tachometer {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 256a256 256 0 1 1 512 0 256 256 0 1 1 -512 0zm320 96c0-16.2-6-31.1-16-42.3l69.5-138.9c5.9-11.9 1.1-26.3-10.7-32.2s-26.3-1.1-32.2 10.7L261.1 288.2c-1.7-.1-3.4-.2-5.1-.2-35.3 0-64 28.7-64 64s28.7 64 64 64 64-28.7 64-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
