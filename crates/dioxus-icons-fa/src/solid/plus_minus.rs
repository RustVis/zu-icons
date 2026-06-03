// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct PlusMinus {}

impl IconShape for PlusMinus {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M224 32c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 128-128 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l128 0 0 128c0 17.7 14.3 32 32 32s32-14.3 32-32l0-128 128 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-128 0 0-128zM0 480c0 17.7 14.3 32 32 32l320 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 448c-17.7 0-32 14.3-32 32z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
