// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct GlassWhiskey {}

impl IconShape for GlassWhiskey {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32C14.3 32 0 46.3 0 64L0 352c0 70.7 57.3 128 128 128l192 0c70.7 0 128-57.3 128-128l0-288c0-17.7-14.3-32-32-32L32 32zM64 256l0-160 320 0 0 160-320 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
