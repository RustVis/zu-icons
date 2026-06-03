// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MaskFaceProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn MaskFace(props: MaskFaceProps) -> Element {
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
        d: "M320 64c-27.2 0-53.8 8-76.4 23.1l-37.1 24.8c-15.8 10.5-34.3 16.1-53.3 16.1L64 128c-35.3 0-64 28.7-64 64l0 78.7c0 62.6 45.2 116 107 126.3l97.9 16.3c33 22 72.6 34.7 115.1 34.7s82.2-12.8 115.1-34.7L533 396.9c61.7-10.3 107-63.7 107-126.3l0-78.7c0-35.3-28.7-64-64-64l-89.2 0c-19 0-37.5-5.6-53.3-16.1L396.4 87.1C373.8 72 347.2 64 320 64zM135.9 336.9l-18.4-3.1C86.6 328.7 64 302 64 270.7l0-78.7 48 0 0 48c0 35 8.6 67.9 23.9 96.9zm386.6-3.1l-18.4 3.1C519.4 307.9 528 275 528 240l0-48 48 0 0 78.7c0 31.3-22.6 58-53.5 63.1zM192 232c0-13.3 10.7-24 24-24l208 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-208 0c-13.3 0-24-10.7-24-24zm56 56l144 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-144 0c-13.3 0-24-10.7-24-24s10.7-24 24-24z",
            }
        }
    }
}
