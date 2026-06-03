// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct MehBlank {}

impl IconShape for MehBlank {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 48a208 208 0 1 0 0 416 208 208 0 1 0 0-416zM512 256a256 256 0 1 1 -512 0 256 256 0 1 1 512 0zM144 208a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm192-32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
