// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Pallet {}

impl IconShape for Pallet {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 320c-17.7 0-32 14.3-32 32s14.3 32 32 32l32 0 0 64-32 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l512 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-32 0 0-64 32 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 320zm96 64l128 0 0 64-128 0 0-64zm192 0l128 0 0 64-128 0 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
