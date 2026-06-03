// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Pixelfed {}

impl IconShape for Pixelfed {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 0a256 256 0 1 1 0 512 256 256 0 1 1 0-512zM235.7 311.9l47 0c44.2 0 80.1-34.9 80.1-78s-35.9-78-80.1-78l-67.8 0c-25.5 0-46.2 20.1-46.2 45l0 175.1 67-64.1z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
