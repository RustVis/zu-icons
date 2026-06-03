// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct FolderTree {}

impl IconShape for FolderTree {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M48 24C48 10.7 37.3 0 24 0S0 10.7 0 24L0 392c0 30.9 25.1 56 56 56l184 0 0-48-184 0c-4.4 0-8-3.6-8-8l0-232 192 0 0-48-192 0 0-88zM336 224l192 0c26.5 0 48-21.5 48-48l0-96c0-26.5-21.5-48-48-48l-82.7 0c-8.5 0-16.6-3.4-22.6-9.4l-8.6-8.6c-9-9-21.2-14.1-33.9-14.1L336 0c-26.5 0-48 21.5-48 48l0 128c0 26.5 21.5 48 48 48zm0 288l192 0c26.5 0 48-21.5 48-48l0-96c0-26.5-21.5-48-48-48l-82.7 0c-8.5 0-16.6-3.4-22.6-9.4l-8.6-8.6c-9-9-21.2-14.1-33.9-14.1L336 288c-26.5 0-48 21.5-48 48l0 128c0 26.5 21.5 48 48 48z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
