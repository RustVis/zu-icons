// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Fan {}

impl IconShape for Fan {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M160 144c0-79.5 64.5-144 144-144 8.8 0 16 7.2 16 16l0 152.2c15-5.3 31.2-8.2 48-8.2 79.5 0 144 64.5 144 144 0 8.8-7.2 16-16 16l-152.2 0c5.3 15 8.2 31.2 8.2 48 0 79.5-64.5 144-144 144-8.8 0-16-7.2-16-16l0-152.2c-15 5.3-31.2 8.2-48 8.2-79.5 0-144-64.5-144-144 0-8.8 7.2-16 16-16l152.2 0c-5.3-15-8.2-31.2-8.2-48zm96 144a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
