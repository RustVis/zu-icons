// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DiggingProps {
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

pub fn Digging(props: DiggingProps) -> Element {
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
        d: "M208 40a56 56 0 1 1 112 0 56 56 0 1 1 -112 0zM10.5 181.3c5.9-11.9 20.3-16.7 32.2-10.7l24.6 12.3 12.2-20.4c18.9-31.5 53.2-50.5 89.6-50.5 46.2 0 87.7 30.5 100.5 75.4l32.2 112.7 92.9 46.4 25.8-43c5.8-9.6 16.2-15.5 27.4-15.5s21.7 5.9 27.4 15.5l96 160c5.9 9.9 6.1 22.2 .4 32.2S555.5 512 544 512l-192 0c-11.5 0-22.2-6.2-27.8-16.2s-5.5-22.3 .4-32.2L370 387.8 21.3 213.5c-11.9-5.9-16.7-20.3-10.7-32.2zM94.3 307.4l112 56c10.8 5.4 17.7 16.5 17.7 28.6l0 88c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-68.2-61.3-30.7-36.3 109c-5.6 16.8-23.7 25.8-40.5 20.2S-3.9 486.6 1.7 469.9l48-144c2.9-8.8 9.5-15.9 18.1-19.4s18.3-3.2 26.6 .9z",
            }
        }
    }
}
