// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Mouse {}

impl IconShape for Mouse {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 192l168 0 0-192-8 0C71.6 0 0 71.6 0 160l0 32zm0 48L0 352c0 88.4 71.6 160 160 160l64 0c88.4 0 160-71.6 160-160l0-112-384 0zm384-48l0-32C384 71.6 312.4 0 224 0l-8 0 0 192 168 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
