// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LariSignProps {
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

pub fn LariSign(props: LariSignProps) -> Element {
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
        d: "M144 0c13.3 0 24 10.7 24 24l0 41.5c7.9-1 15.9-1.5 24-1.5s16.1 .5 24 1.5L216 24c0-13.3 10.7-24 24-24s24 10.7 24 24l0 54c58.9 23.8 103.2 76 116.2 139.7 3.5 17.3-7.7 34.2-25 37.7s-34.2-7.7-37.7-25c-6.7-33.2-26.4-61.8-53.4-80.2l0 81.8c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-101.8c-7.8-1.5-15.8-2.2-24-2.2s-16.2 .8-24 2.2L168 232c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-81.8c-33.8 23-56 61.9-56 105.8 0 70.7 57.3 128 128 128l160 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 448c-17.7 0-32-14.3-32-32s14.3-32 32-32l16.9 0C18.5 350 0 305.2 0 256 0 175.4 49.6 106.4 120 78l0-54c0-13.3 10.7-24 24-24z",
            }
        }
    }
}
