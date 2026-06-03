// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Thermometer3 {}

impl IconShape for Thermometer3 {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M160 0C107 0 64 43 64 96l0 164.7C34.5 287 16 325.4 16 368 16 447.5 80.5 512 160 512s144-64.5 144-144c0-42.6-18.5-81-48-107.3L256 96c0-53-43-96-96-96zm64 368c0 35.3-28.7 64-64 64s-64-28.7-64-64c0-26.9 16.5-49.9 40-59.3L136 152c0-13.3 10.7-24 24-24s24 10.7 24 24l0 156.7c23.5 9.5 40 32.5 40 59.3z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
