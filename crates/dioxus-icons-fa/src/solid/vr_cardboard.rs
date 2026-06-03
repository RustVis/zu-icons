// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct VrCardboard {}

impl IconShape for VrCardboard {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M512 96L64 96C28.7 96 0 124.7 0 160L0 352c0 35.3 28.7 64 64 64l117.5 0c17 0 33.3-6.7 45.3-18.7l33.9-33.9c7.2-7.2 17.1-11.3 27.3-11.3s20.1 4.1 27.3 11.3l33.9 33.9c12 12 28.3 18.7 45.3 18.7L512 416c35.3 0 64-28.7 64-64l0-192c0-35.3-28.7-64-64-64zM80 240a64 64 0 1 1 128 0 64 64 0 1 1 -128 0zm352-64a64 64 0 1 1 0 128 64 64 0 1 1 0-128z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
