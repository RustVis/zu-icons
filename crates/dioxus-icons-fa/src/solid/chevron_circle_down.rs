// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct ChevronCircleDown {}

impl IconShape for ChevronCircleDown {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 0a256 256 0 1 0 0 512 256 256 0 1 0 0-512zM135 241c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l87 87 87-87c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9L273 345c-9.4 9.4-24.6 9.4-33.9 0L135 241z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
