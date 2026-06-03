// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct GlassMartini {}

impl IconShape for GlassMartini {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32C19.1 32 7.4 39.8 2.4 51.8S.2 77.5 9.4 86.6l214.6 214.6 0 146.7-64 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l192 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-64 0 0-146.7 214.6-214.6c9.2-9.2 11.9-22.9 6.9-34.9S492.9 32 480 32L32 32zM256 242.7L109.3 96 402.7 96 256 242.7z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
