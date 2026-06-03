// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct VolumeOff {}

impl IconShape for VolumeOff {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M96 352l-48 0c-26.5 0-48-21.5-48-48l0-96c0-26.5 21.5-48 48-48l48 0 134.1-119.2c6.4-5.7 14.6-8.8 23.1-8.8 19.2 0 34.8 15.6 34.8 34.8l0 378.4c0 19.2-15.6 34.8-34.8 34.8-8.5 0-16.7-3.1-23.1-8.8L96 352z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
