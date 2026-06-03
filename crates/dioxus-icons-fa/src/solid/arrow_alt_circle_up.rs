// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct ArrowAltCircleUp {}

impl IconShape for ArrowAltCircleUp {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zm11.3-387.3l104 104c4.6 4.6 5.9 11.5 3.5 17.4S366.5 256 360 256l-56 0 0 96c0 17.7-14.3 32-32 32l-32 0c-17.7 0-32-14.3-32-32l0-96-56 0c-6.5 0-12.3-3.9-14.8-9.9s-1.1-12.9 3.5-17.4l104-104c6.2-6.2 16.4-6.2 22.6 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
