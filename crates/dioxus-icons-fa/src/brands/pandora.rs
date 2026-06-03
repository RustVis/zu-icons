// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Pandora {}

impl IconShape for Pandora {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M201.3 32L0 32 0 480 120.2 480c11.2 0 20.3-9.1 20.3-20.3l0-86.1 30.1 0c147.1 0 207.3-82.9 207.3-179.9 0-119.7-90.9-161.8-176.5-161.8z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
