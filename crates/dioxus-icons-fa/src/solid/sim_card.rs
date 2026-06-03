// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct SimCard {}

impl IconShape for SimCard {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 64C0 28.7 28.7 0 64 0L258.7 0c17 0 33.3 6.7 45.3 18.7L365.3 80c12 12 18.7 28.3 18.7 45.3L384 448c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zM96 256c-17.7 0-32 14.3-32 32l0 40 128 0 0-72-96 0zM64 416c0 17.7 14.3 32 32 32l48 0 0-72-80 0 0 40zm256 0l0-40-128 0 0 72 96 0c17.7 0 32-14.3 32-32zm0-128c0-17.7-14.3-32-32-32l-48 0 0 72 80 0 0-40z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
