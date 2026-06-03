// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BalanceScaleLeftProps {
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

pub fn BalanceScaleLeft(props: BalanceScaleLeftProps) -> Element {
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
        d: "M519.8 62.4c16.8-5.6 25.8-23.7 20.2-40.5S516.3-3.9 499.6 1.6l-113 37.7c-13.9-23.5-39.6-39.3-68.9-39.3-44.2 0-80 35.8-80 80 0 3 .2 5.9 .5 8.8L115.6 129.6c-16.8 5.6-25.8 23.7-20.2 40.5s23.7 25.8 40.5 20.2l135.5-45.2c4.5 3.2 9.3 5.9 14.4 8.2l0 326.7c0 17.7 14.3 32 32 32l192 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-160 0 0-294.7c21-9.2 37.2-27 44.2-49l125.9-42zM437.3 288l72.4-124.2 72.4 124.2-144.9 0zm72.4 96c62.9 0 115.2-34 126-78.9 2.6-11-1-22.3-6.7-32.1L533.8 109.8c-5-8.6-14.2-13.8-24.1-13.8s-19.1 5.3-24.1 13.8L390.4 273.1c-5.7 9.8-9.3 21.1-6.7 32.1 10.8 44.8 63.1 78.9 126 78.9zM126.8 291.8l72.4 124.2-144.9 0 72.4-124.2zM.9 433.1C11.7 478 64 512 126.8 512s115.2-34 126-78.9c2.6-11-1-22.3-6.7-32.1L150.9 237.8c-5-8.6-14.2-13.8-24.1-13.8s-19.1 5.3-24.1 13.8L7.6 401.1c-5.7 9.8-9.3 21.1-6.7 32.1z",
            }
        }
    }
}
