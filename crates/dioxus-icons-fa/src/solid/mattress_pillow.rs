// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct MattressPillow {}

impl IconShape for MattressPillow {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M256 64L64 64C28.7 64 0 92.7 0 128L0 384c0 35.3 28.7 64 64 64l192 0 0-384zm48 384l208 0c35.3 0 64-28.7 64-64l0-256c0-35.3-28.7-64-64-64l-208 0 0 384zM64 160c0-17.7 14.3-32 32-32l64 0c17.7 0 32 14.3 32 32l0 192c0 17.7-14.3 32-32 32l-64 0c-17.7 0-32-14.3-32-32l0-192z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 576 512");

}
