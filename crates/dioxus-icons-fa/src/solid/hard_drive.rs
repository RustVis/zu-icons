// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct HardDrive {}

impl IconShape for HardDrive {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M64 32C28.7 32 0 60.7 0 96L0 260.1C18.1 247.4 40.2 240 64 240l320 0c23.8 0 45.9 7.4 64 20.1L448 96c0-35.3-28.7-64-64-64L64 32zM448 352c0-35.3-28.7-64-64-64L64 288c-35.3 0-64 28.7-64 64l0 64c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-64zM224 384a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm128-32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 448 512");

}
