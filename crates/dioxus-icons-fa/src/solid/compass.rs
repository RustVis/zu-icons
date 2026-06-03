// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Compass {}

impl IconShape for Compass {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zm50.7-186.9L162.4 380.6c-19.4 7.5-38.5-11.6-31-31l55.5-144.3c3.3-8.5 9.9-15.1 18.4-18.4l144.3-55.5c19.4-7.5 38.5 11.6 31 31L325.1 306.7c-3.2 8.5-9.9 15.1-18.4 18.4zM288 256a32 32 0 1 0 -64 0 32 32 0 1 0 64 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
