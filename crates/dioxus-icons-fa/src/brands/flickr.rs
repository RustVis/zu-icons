// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Flickr {}

impl IconShape for Flickr {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M400 32L48 32C21.5 32 0 53.5 0 80L0 432c0 26.5 21.5 48 48 48l352 0c26.5 0 48-21.5 48-48l0-352c0-26.5-21.5-48-48-48zM144.5 192a63.5 63.5 0 1 1 0 127 63.5 63.5 0 1 1 0-127zm159 0a63.5 63.5 0 1 1 0 127 63.5 63.5 0 1 1 0-127z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
