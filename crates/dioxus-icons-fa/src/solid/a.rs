// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct A {}

impl IconShape for A {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M221.5 51.7C216.6 39.8 204.9 32 192 32s-24.6 7.8-29.5 19.7c-93.3 224-146.7 352-160 384-6.8 16.3 .9 35 17.2 41.8s35-.9 41.8-17.2l31.8-76.3 197.3 0 31.8 76.3c6.8 16.3 25.5 24 41.8 17.2s24-25.5 17.2-41.8c-13.3-32-66.7-160-160-384zM264 320l-144 0 72-172.8 72 172.8z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 384 512");

}
