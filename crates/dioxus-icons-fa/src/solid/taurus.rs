// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Taurus {}

impl IconShape for Taurus {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M336-16c17.7 0 32 14.3 32 32 0 58.5-28.6 110.3-72.5 142.3 53.2 34.1 88.5 93.8 88.5 161.7 0 106-86 192-192 192S0 426 0 320C0 252.1 35.3 192.4 88.5 158.3 44.6 126.3 16 74.5 16 16 16-1.7 30.3-16 48-16S80-1.7 80 16c0 61.9 50.1 112 112 112S304 77.9 304 16c0-17.7 14.3-32 32-32zM192 192a128 128 0 1 0 0 256 128 128 0 1 0 0-256z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
