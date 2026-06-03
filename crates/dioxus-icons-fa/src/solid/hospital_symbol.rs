// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct HospitalSymbol {}

impl IconShape for HospitalSymbol {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zm96-344l0 176c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-64-96 0 0 64c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-176c0-13.3 10.7-24 24-24s24 10.7 24 24l0 64 96 0 0-64c0-13.3 10.7-24 24-24s24 10.7 24 24z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
