// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct Bandage {}

impl IconShape for Bandage {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M464 416l48 0c35.3 0 64-28.7 64-64l0-192c0-35.3-28.7-64-64-64l-48 0 0 320zM416 96l-256 0 0 320 256 0 0-320zM64 96C28.7 96 0 124.7 0 160L0 352c0 35.3 28.7 64 64 64l48 0 0-320-48 0zM216 208a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm120-24a24 24 0 1 1 0 48 24 24 0 1 1 0-48zM216 304a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm120-24a24 24 0 1 1 0 48 24 24 0 1 1 0-48z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
