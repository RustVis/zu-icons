// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct TruckFront {}

impl IconShape for TruckFront {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 112C0 67.8 35.8 32 80 32l288 0c44.2 0 80 35.8 80 80l0 256c0 26.2-12.6 49.4-32 64l0 48c0 17.7-14.3 32-32 32l-32 0c-17.7 0-32-14.3-32-32l0-32-192 0 0 32c0 17.7-14.3 32-32 32l-32 0c-17.7 0-32-14.3-32-32l0-48C12.6 417.4 0 394.2 0 368L0 112zm96 80l0 64 256 0 0-64c0-17.7-14.3-32-32-32l-192 0c-17.7 0-32 14.3-32 32zm32 160a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm224 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
