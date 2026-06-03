// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Chalkboard {}

impl IconShape for Chalkboard {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M96 64c-35.3 0-64 28.7-64 64l0 256c-17.7 0-32 14.3-32 32s14.3 32 32 32l512 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l0-256c0-35.3-28.7-64-64-64L96 64zM480 384l-64 0 0-32c0-17.7-14.3-32-32-32l-96 0c-17.7 0-32 14.3-32 32l0 32-160 0 0-256 384 0 0 256z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
