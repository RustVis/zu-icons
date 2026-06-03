// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct TabletButton {}

impl IconShape for TabletButton {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M64 0C28.7 0 0 28.7 0 64L0 448c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-384c0-35.3-28.7-64-64-64L64 0zM224 400a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
