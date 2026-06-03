// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::{Element, dioxus_core, dioxus_elements, dioxus_signals, rsx};

use crate::IconShape;

#[derive(Clone, PartialEq)]
pub struct YinYang {}

impl IconShape for YinYang {
    fn child_elements(&self) -> Element {
        rsx!(path {
        d: "M224 160a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm32 352a256 256 0 1 0 0-512 256 256 0 1 0 0 512zm0-448c53 0 96 43 96 96s-43 96-96 96-96 43-96 96 43 96 96 96C150 448 64 362 64 256S150 64 256 64zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z",
            })
    }

    const VIEW_BOX: Option<&'static str> = Some("0 0 512 512");

}
