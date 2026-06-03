// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Neuter {}

impl IconShape for Neuter {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M80 176a112 112 0 1 1 224 0 112 112 0 1 1 -224 0zM223.9 349.1C305.9 334.1 368 262.3 368 176 368 78.8 289.2 0 192 0S16 78.8 16 176c0 86.3 62.1 158.1 144.1 173.1-.1 1-.1 1.9-.1 2.9l0 160c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160c0-1 0-1.9-.1-2.9z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
