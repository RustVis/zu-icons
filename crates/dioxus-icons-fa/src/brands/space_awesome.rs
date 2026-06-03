// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct SpaceAwesome {}

impl IconShape for SpaceAwesome {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M96 256l32 0 0 256-128 0 0-160 32 0 0-32 32 0 0-32 32 0 0-32zm416 96l0 160-128 0 0-256 32 0 0 32 32 0 0 32 32 0 0 32 32 0zM320 64l32 0 0 384-32 0 0-32-128 0 0 32-32 0 0-384 32 0 0-32 32 0 0-32 64 0 0 32 32 0 0 32zm-32 64l-64 0 0 64 64 0 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
