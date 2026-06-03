// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct ExternalLinkSquare {}

impl IconShape for ExternalLinkSquare {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M384 32c35.3 0 64 28.7 64 64l0 320c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64L0 96C0 60.7 28.7 32 64 32l320 0zM272 296c0 13.3 10.7 24 24 24s24-10.7 24-24l0-112c0-13.3-10.7-24-24-24l-112 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l54.1 0-103 103c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l103-103 0 54.1z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
