// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Magnet {}

impl IconShape for Magnet {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 176L0 288C0 411.7 100.3 512 224 512S448 411.7 448 288l0-112-128 0 0 112c0 53-43 96-96 96s-96-43-96-96l0-112-128 0zm0-48l128 0 0-64c0-17.7-14.3-32-32-32L32 32C14.3 32 0 46.3 0 64l0 64zm320 0l128 0 0-64c0-17.7-14.3-32-32-32l-64 0c-17.7 0-32 14.3-32 32l0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
