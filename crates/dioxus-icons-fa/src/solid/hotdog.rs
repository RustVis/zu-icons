// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Hotdog {}

impl IconShape for Hotdog {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M288 0c-20.5 0-40.1 8.1-54.6 22.6L22.6 233.4C8.1 247.9 0 267.5 0 288 0 300.2 2.9 312.1 8.2 322.7L322.7 8.2C312.1 2.9 300.2 0 288 0zM224 512c20.5 0 40.1-8.1 54.6-22.6L489.4 278.6c14.5-14.5 22.6-34.1 22.6-54.6 0-12.2-2.9-24.1-8.2-34.7L189.3 503.8c10.7 5.4 22.6 8.2 34.7 8.2zM456.6 168.6c31.2-31.2 31.2-81.9 0-113.1s-81.9-31.2-113.1 0l-288 288c-31.2 31.2-31.2 81.9 0 113.1s81.9 31.2 113.1 0l288-288z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
