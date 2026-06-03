// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct D {}

impl IconShape for D {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 64C0 46.3 14.3 32 32 32l128 0c123.7 0 224 100.3 224 224S283.7 480 160 480L32 480c-17.7 0-32-14.3-32-32L0 64zM64 96l0 320 96 0c88.4 0 160-71.6 160-160S248.4 96 160 96L64 96z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
