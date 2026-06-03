// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Npm {}

impl IconShape for Npm {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M288 288l-32 0 0-64 32 0 0 64zM576 160l0 192-288 0 0 32-128 0 0-32-160 0 0-192 576 0zM160 192l-128 0 0 128 64 0 0-96 32 0 0 96 32 0 0-128zm160 0l-128 0 0 160 64 0 0-32 64 0 0-128zm224 0l-192 0 0 128 64 0 0-96 32 0 0 96 32 0 0-96 32 0 0 96 32 0 0-128z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
