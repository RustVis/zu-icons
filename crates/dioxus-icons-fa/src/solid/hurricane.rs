// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Hurricane {}

impl IconShape for Hurricane {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 208C0 104.4 75.7 18.5 174.9 2.6 184 1.2 192 8.6 192 17.9l0 63.3c0 8.4 6.5 15.3 14.7 16.5 100.3 14.9 177.3 101.3 177.3 205.7 0 103.6-75.7 189.5-174.9 205.4-9.2 1.5-17.1-5.9-17.1-15.2l0-63.3C192 421.9 185.5 415 177.3 413.7 77 398.9 0 312.4 0 208zm288 48a96 96 0 1 0 -192 0 96 96 0 1 0 192 0zm-96-32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
