// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct HackerNews {}

impl IconShape for HackerNews {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 32l0 448 448 0 0-448-448 0zM21.2 229.2l-.2 0c.1-.1 .2-.3 .3-.4 0 .1 0 .3-.1 .4zm218 53.9l0 100.9-31.4 0 0-102.7-79.8-153.3 37.3 0c52.5 98.3 49.2 101.2 59.3 125.6 12.3-27 5.8-24.4 60.6-125.6l34.8 0-80.8 155.1z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
