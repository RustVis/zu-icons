// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PesetaSignProps {
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

pub fn PesetaSign(props: PesetaSignProps) -> Element {
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
        d: "M112 32C94.3 32 80 46.3 80 64l0 104-24 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l24 0 0 232c0 17.7 14.3 32 32 32s32-14.3 32-32l0-96 96 0c80.2 0 146.6-59 158.2-136l25.8 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-25.8 0C386.6 91 320.2 32 240 32L112 32zM333 168l-189 0 0-72 96 0c44.7 0 82.3 30.6 93 72zM144 216l189 0c-10.7 41.4-48.2 72-93 72l-96 0 0-72z",
            }
        }
    }
}
