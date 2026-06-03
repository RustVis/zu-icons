// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Comment {}

impl IconShape for Comment {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M512 240c0 132.5-114.6 240-256 240-37.1 0-72.3-7.4-104.1-20.7L33.5 510.1c-9.4 4-20.2 1.7-27.1-5.8S-2 485.8 2.8 476.8l48.8-92.2C19.2 344.3 0 294.3 0 240 0 107.5 114.6 0 256 0S512 107.5 512 240z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
