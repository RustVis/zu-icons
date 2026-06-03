// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Worm {}

impl IconShape for Worm {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M224 96c0-53 43-96 96-96l38.4 0C407.9 0 448 40.1 448 89.6L448 376c0 75.1-60.9 136-136 136S176 451.1 176 376l0-80c0-22.1-17.9-40-40-40s-40 17.9-40 40l0 168c0 26.5-21.5 48-48 48S0 490.5 0 464L0 296c0-75.1 60.9-136 136-136s136 60.9 136 136l0 80c0 22.1 17.9 40 40 40s40-17.9 40-40l0-184-32 0c-53 0-96-43-96-96zm144-8a24 24 0 1 0 -48 0 24 24 0 1 0 48 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
