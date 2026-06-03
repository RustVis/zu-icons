// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct ArrowAltCircleDown {}

impl IconShape for ArrowAltCircleDown {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 464a208 208 0 1 1 0-416 208 208 0 1 1 0 416zM256 0a256 256 0 1 0 0 512 256 256 0 1 0 0-512zM244.7 387.3c6.2 6.2 16.4 6.2 22.6 0l104-104c4.6-4.6 5.9-11.5 3.5-17.4S366.5 256 360 256l-72 0 0-104c0-13.3-10.7-24-24-24l-16 0c-13.3 0-24 10.7-24 24l0 104-72 0c-6.5 0-12.3 3.9-14.8 9.9s-1.1 12.9 3.5 17.4l104 104z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
