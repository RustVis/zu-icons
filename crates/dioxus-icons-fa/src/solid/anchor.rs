// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AnchorProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Anchor(props: AnchorProps) -> Element {
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
        d: "M288 64a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM192 96c0-53 43-96 96-96s96 43 96 96c0 41.8-26.7 77.4-64 90.5l0 257.9c62.9-14.3 110.2-69.7 111.9-136.5l-16.1 14.1c-10 8.7-25.1 7.7-33.9-2.3s-7.7-25.1 2.3-33.9l64-56c9-7.9 22.6-7.9 31.6 0l64 56c10 8.7 11 23.9 2.3 33.9s-23.9 11-33.9 2.3L496 307.9C493.9 421 401.6 512 288 512S82.1 421 80 307.9L63.8 322.1c-10 8.7-25.1 7.7-33.9-2.3s-7.7-25.1 2.3-33.9l64-56c9-7.9 22.6-7.9 31.6 0l64 56c10 8.7 11 23.9 2.3 33.9s-23.9 11-33.9 2.3l-16.1-14.1c1.8 66.8 49.1 122.2 111.9 136.5l0-257.9c-37.3-13.2-64-48.7-64-90.5z",
            }
        }
    }
}
