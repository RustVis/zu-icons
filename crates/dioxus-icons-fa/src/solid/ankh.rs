// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Ankh {}

impl IconShape for Ankh {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M96 128c0-35.3 28.7-64 64-64s64 28.7 64 64c0 41.6-20.7 76.6-46.6 104.1-5.9 6.2-11.8 11.8-17.4 16.7-5.6-4.9-11.5-10.5-17.4-16.7-25.9-27.5-46.6-62.6-46.6-104.1zM160 0C89.3 0 32 57.3 32 128 32 180.4 53.5 223.5 78.8 256L32 256c-17.7 0-32 14.3-32 32s14.3 32 32 32l96 0 0 160c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160 96 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-46.8 0C266.5 223.5 288 180.4 288 128 288 57.3 230.7 0 160 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 320 512");

}
