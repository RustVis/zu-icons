// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Diamond {}

impl IconShape for Diamond {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M17 215L215 17C225.9 6.1 240.6 0 256 0s30.1 6.1 41 17L495 215c10.9 10.9 17 25.6 17 41s-6.1 30.1-17 41L297 495c-10.9 10.9-25.6 17-41 17s-30.1-6.1-41-17L17 297C6.1 286.1 0 271.4 0 256s6.1-30.1 17-41z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
