// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct RulerHorizontal {}

impl IconShape for RulerHorizontal {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M48 384c-26.5 0-48-21.5-48-48L0 176c0-26.5 21.5-48 48-48l24 0 0 104c0 13.3 10.7 24 24 24s24-10.7 24-24l0-104 48 0 0 72c0 13.3 10.7 24 24 24s24-10.7 24-24l0-72 48 0 0 104c0 13.3 10.7 24 24 24s24-10.7 24-24l0-104 48 0 0 72c0 13.3 10.7 24 24 24s24-10.7 24-24l0-72 48 0 0 104c0 13.3 10.7 24 24 24s24-10.7 24-24l0-104 24 0c26.5 0 48 21.5 48 48l0 160c0 26.5-21.5 48-48 48L48 384z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
