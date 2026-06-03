// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Prescription {}

impl IconShape for Prescription {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M32 0C14.3 0 0 14.3 0 32L0 288c0 17.7 14.3 32 32 32s32-14.3 32-32l0-64 50.7 0 128 128-105.4 105.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L288 397.3 393.4 502.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L333.3 352 438.6 246.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L288 306.7 202.2 220.9C251.4 209.1 288 164.8 288 112 288 50.1 237.9 0 176 0L32 0zM176 160l-112 0 0-96 112 0c26.5 0 48 21.5 48 48s-21.5 48-48 48z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
