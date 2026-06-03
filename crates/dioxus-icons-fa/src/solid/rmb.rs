// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RmbProps {
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

pub fn Rmb(props: RmbProps) -> Element {
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
        d: "M74.9 46.7c-9.6-14.9-29.4-19.2-44.2-9.6S11.5 66.4 21.1 81.3L143.7 272 88 272c-13.3 0-24 10.7-24 24s10.7 24 24 24l72 0 0 32-72 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l72 0 0 48c0 17.7 14.3 32 32 32s32-14.3 32-32l0-48 72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-72 0 0-32 72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-55.7 0 122.6-190.7c9.6-14.9 5.3-34.7-9.6-44.2s-34.7-5.3-44.2 9.6L192 228.8 74.9 46.7z",
            }
        }
    }
}
