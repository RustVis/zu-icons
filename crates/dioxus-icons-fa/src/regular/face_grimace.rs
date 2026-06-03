// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct FaceGrimace {}

impl IconShape for FaceGrimace {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 48a208 208 0 1 0 0 416 208 208 0 1 0 0-416zM512 256a256 256 0 1 1 -512 0 256 256 0 1 1 512 0zM152 352c0 11.9 8.6 21.8 20 23.7l0-47.3c-11.4 1.9-20 11.8-20 23.7zm84 24l0-48-24 0 0 48 24 0zm64 0l0-48-24 0 0 48 24 0zm40-.3c11.4-1.9 20-11.8 20-23.7s-8.6-21.8-20-23.7l0 47.3zM176 288l160 0c35.3 0 64 28.7 64 64s-28.7 64-64 64l-160 0c-35.3 0-64-28.7-64-64s28.7-64 64-64zm0-112a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm128 32a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
