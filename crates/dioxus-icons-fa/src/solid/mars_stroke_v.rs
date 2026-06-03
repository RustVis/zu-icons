// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MarsStrokeVProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn MarsStrokeV(props: MarsStrokeVProps) -> Element {
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
        d: "M192 256a112 112 0 1 0 0 224 112 112 0 1 0 0-224zM16 368c0-86.3 62.1-158.1 144.1-173.1-.1-.9-.1-1.9-.1-2.9l0-16-32 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l32 0 0-61.4-28 22.4c-13.8 11-33.9 8.8-45-5s-8.8-33.9 5-45l80-64c11.7-9.3 28.3-9.3 40 0l80 64c13.8 11 16 31.2 5 45s-31.2 16-45 5l-28-22.4 0 61.4 32 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-32 0 0 16c0 1 0 1.9-.1 2.9 82 15 144.1 86.8 144.1 173.1 0 97.2-78.8 176-176 176S16 465.2 16 368z",
            }
        }
    }
}
