// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Road {}

impl IconShape for Road {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M223.9 32l-76.2 0c-29.4 0-55.1 20.1-62.1 48.6L1.4 420.5C-6.1 450.7 16.8 480 48 480l175.9 0 0-64c0-17.7 14.3-32 32-32s32 14.3 32 32l0 64 176.1 0c31.2 0 54.1-29.3 46.6-59.5L426.5 80.6C419.4 52.1 393.8 32 364.3 32l-76.4 0 0 64c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-64zm64 192l0 64c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-64c0-17.7 14.3-32 32-32s32 14.3 32 32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
