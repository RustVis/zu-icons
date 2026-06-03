// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CarOnProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn CarOn(props: CarOnProps) -> Element {
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
        d: "M248-8c0-13.3-10.7-24-24-24S200-21.3 200-8l0 64c0 13.3 10.7 24 24 24s24-10.7 24-24l0-64zM145.3 208l157.5 0c6.7 0 12.6 4.1 15 10.4l26.1 69.6-239.6 0 26.1-69.6c2.3-6.2 8.3-10.4 15-10.4zM34 292.8l-1.3 3.4C13.2 307.1 0 328 0 352L0 480c0 17.7 14.3 32 32 32l16 0c17.7 0 32-14.3 32-32l0-32 288 0 0 32c0 17.7 14.3 32 32 32l16 0c17.7 0 32-14.3 32-32l0-128c0-24-13.2-44.9-32.8-55.9l-1.3-3.4-36.3-96.9c-11.7-31.2-41.6-51.9-74.9-51.9l-157.5 0c-33.3 0-63.2 20.7-74.9 51.9L34 292.8zM96 336a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm224 32a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zM7 7C-2.3 16.4-2.3 31.6 7 41L55 89c9.4 9.4 24.6 9.4 33.9 0S98.3 64.4 89 55L41 7C31.6-2.3 16.4-2.3 7 7zM407 7L359 55c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l48-48c9.4-9.4 9.4-24.6 0-33.9S416.4-2.3 407 7z",
            }
        }
    }
}
