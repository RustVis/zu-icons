// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct FolderClosed {}

impl IconShape for FolderClosed {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M448 448L64 448c-35.3 0-64-28.7-64-64l0-176 512 0 0 176c0 35.3-28.7 64-64 64zm64-288L0 160 0 96C0 60.7 28.7 32 64 32l138.7 0c13.8 0 27.3 4.5 38.4 12.8l38.4 28.8c5.5 4.2 12.3 6.4 19.2 6.4L448 80c35.3 0 64 28.7 64 64l0 16z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
