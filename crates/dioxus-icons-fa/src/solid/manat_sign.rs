// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct ManatSign {}

impl IconShape for ManatSign {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M192 32c-13.3 0-24 10.7-24 24l0 41.5C73.3 109.3 0 190.1 0 288L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160c0-62.5 44.8-114.5 104-125.8L168 456c0 13.3 10.7 24 24 24s24-10.7 24-24l0-293.8c59.2 11.2 104 63.3 104 125.8l0 160c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160c0-97.9-73.3-178.7-168-190.5L216 56c0-13.3-10.7-24-24-24z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
