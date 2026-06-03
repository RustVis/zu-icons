// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Rouble {}

impl IconShape for Rouble {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M112 32C94.3 32 80 46.3 80 64l0 208-40 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l40 0 0 48-40 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l40 0 0 32c0 17.7 14.3 32 32 32s32-14.3 32-32l0-32 152 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-152 0 0-48 112 0c79.5 0 144-64.5 144-144S335.5 32 256 32L112 32zM256 256l-112 0 0-160 112 0c44.2 0 80 35.8 80 80s-35.8 80-80 80z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
