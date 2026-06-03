// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Ello {}

impl IconShape for Ello {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zM399.8 293.2C383.3 358.5 323.8 404.8 256 404.8S128.7 358.5 112.2 293.2c-1.6-7.4 2.5-15.7 9.9-17.4s15.7 2.5 17.4 9.9c14 52.9 62 90.1 116.6 90.1s102.5-37.2 116.6-90.1c1.7-7.4 9.9-12.4 17.4-9.9 7.4 1.7 12.4 9.9 9.9 17.4z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
