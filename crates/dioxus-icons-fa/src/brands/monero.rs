// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Monero {}

impl IconShape for Monero {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M360 384l108.4 0C425 455.9 346.1 504 256 504S87 455.9 43.6 384l108.4 0 0-127.8 104 104.8 104-105 0 128zM96 336l0-208 159.4 159.4 160.6-159.4 0 208 74.8 0c8.5-25.1 13.2-52 13.2-80 0-137-111-248-248-248S8 119 8 256c0 28 4.6 54.9 13.2 80L96 336z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
