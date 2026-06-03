// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VaultProps {
    #[props(default = None)]
    pub title: Option<&'static str>,

    #[props(default = None)]
    pub class: Option<&'static str>,

    #[props(default = None)]
    pub style: Option<&'static str>,

    #[props(default = None)]
    pub width: Option<&'static str>,

    #[props(default = None)]
    pub height: Option<&'static str>,

    #[props(default = None)]
    pub fill: Option<&'static str>,

    #[props(default = None)]
    pub stroke: Option<&'static str>,

    #[props(default = Some("0 0 512 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Vault(props: VaultProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            style: props.style,
            height: props.height,
            width: props.width,
            view_box: props.view_box.unwrap_or("0 0 16 16"),
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg"),
            fill: props.fill.unwrap_or("currentColor"),
            stroke: props.stroke,

            if let Some(title_text) = props.title {
                title {{ title_text }}
            }

                        path {
        d: "M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64 0 17.7 14.3 32 32 32s32-14.3 32-32l256 0c0 17.7 14.3 32 32 32s32-14.3 32-32c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM256 256a64 64 0 1 0 -128 0 64 64 0 1 0 128 0zM64 256a128 128 0 1 1 256 0 128 128 0 1 1 -256 0zm384-48c0 17.8-9.7 33.3-24 41.6l0 78.4c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-78.4c-14.3-8.3-24-23.8-24-41.6 0-26.5 21.5-48 48-48s48 21.5 48 48z",
            }
        }
    }
}
