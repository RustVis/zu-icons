// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Egg {}

impl IconShape for Egg {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M192 496C86 496 0 394 0 288 0 176 64 16 192 16S384 176 384 288c0 106-86 208-192 208zM154.8 134c6.5-6 7-16.1 1-22.6s-16.1-7-22.6-1c-23.9 21.8-41.1 52.7-52.3 84.2-11.2 31.6-16.9 65.1-16.9 93.5 0 8.8 7.2 16 16 16s16-7.2 16-16c0-24.5 5-54.4 15.1-82.8 10.1-28.5 25-54.1 43.7-71.2z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
