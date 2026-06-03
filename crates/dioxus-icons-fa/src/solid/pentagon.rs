// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Pentagon {}

impl IconShape for Pentagon {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M316.4-3.1c-16.8-12.2-39.6-12.2-56.4 0L35.3 160.2c-16.8 12.2-23.9 33.9-17.4 53.7l85.8 264.1c6.4 19.8 24.9 33.2 45.7 33.2l277.7 0c20.8 0 39.2-13.4 45.7-33.2l85.8-264.1c6.4-19.8-.6-41.4-17.4-53.7L316.4-3.1z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
