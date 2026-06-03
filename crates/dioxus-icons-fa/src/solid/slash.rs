// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Slash {}

impl IconShape for Slash {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M7-25c9.4-9.4 24.6-9.4 33.9 0L569 503c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0L7 9C-2.3-.4-2.3-15.6 7-25z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
