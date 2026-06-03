// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct FaceMehBlank {}

impl IconShape for FaceMehBlank {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 256a256 256 0 1 1 512 0 256 256 0 1 1 -512 0zm208-48a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm128 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
