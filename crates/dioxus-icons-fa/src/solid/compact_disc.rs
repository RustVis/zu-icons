// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct CompactDisc {}

impl IconShape for CompactDisc {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 256a256 256 0 1 1 512 0 256 256 0 1 1 -512 0zm256 32a32 32 0 1 1 0-64 32 32 0 1 1 0 64zm-96-32a96 96 0 1 0 192 0 96 96 0 1 0 -192 0zm-56-16c0-32.4 16.3-66.6 42.8-93.2S207.6 104 240 104c13.3 0 24-10.7 24-24s-10.7-24-24-24c-47.9 0-93.7 23.5-127.1 56.9S56 192.1 56 240c0 13.3 10.7 24 24 24s24-10.7 24-24z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
