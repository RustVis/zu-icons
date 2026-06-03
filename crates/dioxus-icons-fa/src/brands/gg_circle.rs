// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct GgCircle {}

impl IconShape for GgCircle {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M257.5 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zM208 382.8l-125.7-125.7 125.7-125.7 35.2 35.4-24.2 24.2-11.1-11.1-77.2 77.2 77.2 77.2 26.6-26.6-53.1-52.9 24.4-24.4 77.2 77.2-75 75.2zm99-2.2l-35.2-35.2 24.1-24.4 11.1 11.1 77.2-77.2-77.2-77.2-26.5 26.5 53.1 52.9-24.4 24.4-77.2-77.2 75-75 125.7 125.7-125.7 125.6z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
