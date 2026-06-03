// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct WindowMinimize {}

impl IconShape for WindowMinimize {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 424c0-13.3 10.7-24 24-24l464 0c13.3 0 24 10.7 24 24s-10.7 24-24 24L24 448c-13.3 0-24-10.7-24-24z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
