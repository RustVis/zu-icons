// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Viacoin {}

impl IconShape for Viacoin {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M384 32l-64 0-80.7 192-94.5 0-80.8-192-64 0 48 112-48 0 0 48 68.5 0 13.8 32-82.3 0 0 48 102.8 0 89.2 208 89.2-208 102.8 0 0-48-82.3 0 13.8-32 68.5 0 0-48-48 0 48-112zM192 336l-27-64 54 0-27 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
