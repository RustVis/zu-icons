// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct RoadBarrier {}

impl IconShape for RoadBarrier {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32C14.3 32 0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 266.3 149.2 96 64 96 64 64c0-17.7-14.3-32-32-32zM405.2 96l-74.3 0-5.4 10.7-90.6 181.3 74.3 0 5.4-10.7 90.6-181.3zM362.8 288l74.3 0 5.4-10.7 90.6-181.3-74.3 0-5.4 10.7-90.6 181.3zM202.8 96l-5.4 10.7-90.6 181.3 74.3 0 5.4-10.7 90.6-181.3-74.3 0zm288 192l85.2 0 0 160c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 53.7-85.2 170.3z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 640 512");

}
