// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Notdef {}

impl IconShape for Notdef {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M64 390.3L153.5 256 64 121.7 64 390.3zM102.5 448L281.5 448 192 313.7 102.5 448zm128-192L320 390.3 320 121.7 230.5 256zM281.5 64L102.5 64 192 198.3 281.5 64zM0 48C0 21.5 21.5 0 48 0L336 0c26.5 0 48 21.5 48 48l0 416c0 26.5-21.5 48-48 48L48 512c-26.5 0-48-21.5-48-48L0 48z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
