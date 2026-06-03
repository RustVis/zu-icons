// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct PrescriptionBottle {}

impl IconShape for PrescriptionBottle {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M0 32C0 14.3 14.3 0 32 0L352 0c17.7 0 32 14.3 32 32l0 32c0 17.7-14.3 32-32 32L32 96C14.3 96 0 81.7 0 64L0 32zM32 144l320 0 0 304c0 35.3-28.7 64-64 64L96 512c-35.3 0-64-28.7-64-64l72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-72 0 0-48 72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-72 0 0-48 72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-72 0 0-64z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
