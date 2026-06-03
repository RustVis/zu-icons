// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Braille {}

impl IconShape for Braille {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M80 144a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0 192a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0 136c-13.3 0-24-10.7-24-24s10.7-24 24-24 24 10.7 24 24-10.7 24-24 24zm0 56c44.2 0 80-35.8 80-80s-35.8-80-80-80-80 35.8-80 80 35.8 80 80 80zm248-80c0 13.3-10.7 24-24 24s-24-10.7-24-24 10.7-24 24-24 24 10.7 24 24zm56 0c0-44.2-35.8-80-80-80s-80 35.8-80 80 35.8 80 80 80 80-35.8 80-80zM304 232a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm0 104a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0-192a80 80 0 1 0 0-160 80 80 0 1 0 0 160z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
