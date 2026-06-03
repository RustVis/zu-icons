// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct HockeyPuck {}

impl IconShape for HockeyPuck {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 256C114.6 256 0 213 0 160s114.6-96 256-96 256 43 256 96-114.6 96-256 96zM0 352L0 242.7c16.9 12.3 37 22.2 58.1 30.1 53 19.9 123 31.2 197.9 31.2s144.9-11.3 197.9-31.2c21.2-7.9 41.2-17.8 58.1-30.1L512 352c0 53-114.6 96-256 96S0 405 0 352z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
