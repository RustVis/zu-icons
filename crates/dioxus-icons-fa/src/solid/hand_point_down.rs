// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HandPointDownProps {
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

pub fn HandPointDown(props: HandPointDownProps) -> Element {
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
        d: "M32 480c0 17.7 14.3 32 32 32s32-14.3 32-32l0-208-64 0 0 208zM224 320c0 17.7 14.3 32 32 32s32-14.3 32-32l0-64c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 64zm-64 64c17.7 0 32-14.3 32-32l0-48c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 48c0 17.7 14.3 32 32 32zm160-96c0 17.7 14.3 32 32 32s32-14.3 32-32l0-64c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 64zm-96-88l0 .6c9.4-5.4 20.3-8.6 32-8.6 13.2 0 25.4 4 35.6 10.8 8.7-24.9 32.5-42.8 60.4-42.8 11.7 0 22.6 3.1 32 8.6l0-8.6C384 71.6 312.4 0 224 0L162.3 0C119.8 0 79.1 16.9 49.1 46.9L37.5 58.5C13.5 82.5 0 115.1 0 149l0 27c0 35.3 28.7 64 64 64l88 0c22.1 0 40-17.9 40-40s-17.9-40-40-40l-56 0c-8.8 0-16-7.2-16-16s7.2-16 16-16l56 0c39.8 0 72 32.2 72 72z",
            }
        }
    }
}
