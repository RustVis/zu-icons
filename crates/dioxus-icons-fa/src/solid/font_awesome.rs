// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct FontAwesome {}

impl IconShape for FontAwesome {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M91.7 96C106.3 86.8 116 70.5 116 52 116 23.3 92.7 0 64 0S12 23.3 12 52c0 16.7 7.8 31.5 20 41l0 419 64 0 0-64 373.6 0c14.6 0 26.4-11.8 26.4-26.4 0-3.7-.8-7.3-2.3-10.7L432 272 493.7 133.1c1.5-3.4 2.3-7 2.3-10.7 0-14.6-11.8-26.4-26.4-26.4L91.7 96z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
