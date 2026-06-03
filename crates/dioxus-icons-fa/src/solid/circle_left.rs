// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct CircleLeft {}

impl IconShape for CircleLeft {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M512 256a256 256 0 1 0 -512 0 256 256 0 1 0 512 0zM124.7 244.7l104-104c4.6-4.6 11.5-5.9 17.4-3.5s9.9 8.3 9.9 14.8l0 56 96 0c17.7 0 32 14.3 32 32l0 32c0 17.7-14.3 32-32 32l-96 0 0 56c0 6.5-3.9 12.3-9.9 14.8s-12.9 1.1-17.4-3.5l-104-104c-6.2-6.2-6.2-16.4 0-22.6z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
