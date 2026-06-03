// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SynagogueProps {
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

pub fn Synagogue(props: SynagogueProps) -> Element {
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
        d: "M16 80c0-35.3 28.7-64 64-64s64 28.7 64 64l0 32-128 0 0-32zm0 368l0-288 128 0 126.2-84.2c10.7-7.2 24.8-7.2 35.5 0l126.2 84.2 128 0 0 288c0 35.3-28.7 64-64 64L80 512c-35.3 0-64-28.7-64-64zM560 112l-128 0 0-32c0-35.3 28.7-64 64-64s64 28.7 64 64l0 32zM224 384l0 80 128 0 0-80c0-35.3-28.7-64-64-64s-64 28.7-64 64zm64-152a40 40 0 1 0 0-80 40 40 0 1 0 0 80z",
            }
        }
    }
}
