// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LeoProps {
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

    #[props(default = Some("0 0 512 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Leo(props: LeoProps) -> Element {
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
        d: "M260 0c72.9 0 132 59.1 132 132l0 5.4-.1 1.4-23.8 270c.4 21.7 18.2 39.2 40 39.2 22.1 0 40-17.9 40-40l0-40c0-17.7 14.3-32 32-32s32 14.3 32 32l0 40c0 57.4-46.6 104-104 104S304 465.4 304 408l0-1.4 .1-1.4 23.9-270.6 0-2.6c0-37.6-30.4-68-68-68s-68 30.4-68 68l0 4c0 3.7 .3 7.3 .8 11l29.7 193.4c.9 6.1 1.4 12.2 1.4 18.4l0 9.2c0 61.9-50.1 112-112 112S0 429.9 0 368 50.1 256 112 256c11.7 0 23 1.8 33.7 5.1L129.6 156.7c-1.1-6.8-1.6-13.8-1.6-20.7l0-4C128 59.1 187.1 0 260 0zM112 320a48 48 0 1 0 0 96 48 48 0 1 0 0-96z",
            }
        }
    }
}
