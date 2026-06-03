// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Archway {}

impl IconShape for Archway {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l448 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 32zm0 384c-17.7 0-32 14.3-32 32s14.3 32 32 32l128 0 0-128c0-53 43-96 96-96s96 43 96 96l0 128 128 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l0-272-448 0 0 272z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
