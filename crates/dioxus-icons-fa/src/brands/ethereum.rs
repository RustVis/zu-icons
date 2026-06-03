// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Ethereum {}

impl IconShape for Ethereum {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M311.9 260.8L160 353.6 8 260.8 160 0 311.9 260.8zM160 383.4L8 290.6 160 512 312 290.6 160 383.4z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
