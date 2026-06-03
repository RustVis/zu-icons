// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Mask {}

impl IconShape for Mask {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M288 64C64 64 0 160 0 272S80 448 176 448l8.4 0c24.2 0 46.4-13.7 57.2-35.4l23.2-46.3c4.4-8.8 13.3-14.3 23.2-14.3s18.8 5.5 23.2 14.3l23.2 46.3c10.8 21.7 33 35.4 57.2 35.4l8.4 0c96 0 176-64 176-176S512 64 288 64zM96 256a64 64 0 1 1 128 0 64 64 0 1 1 -128 0zm320-64a64 64 0 1 1 0 128 64 64 0 1 1 0-128z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
