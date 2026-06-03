// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PiggyBankProps {
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

pub fn PiggyBank(props: PiggyBankProps) -> Element {
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
        d: "M288-32a96 96 0 1 1 0 192 96 96 0 1 1 0-192zM48 304c0-70.1 47-131.4 117.1-164.9 25.3 41.3 70.9 68.9 122.9 68.9 55.7 0 104.1-31.7 128-78 15.8-11.3 35.1-18 56-18l19.5 0c10.4 0 18 9.8 15.5 19.9l-17.1 68.3c9.9 12.4 18.2 25.7 24.4 39.8l21.7 0c13.3 0 24 10.7 24 24l0 112c0 13.3-10.7 24-24 24l-40 0c-16.5 22-38.5 39.6-64 50.7l0 29.3c0 17.7-14.3 32-32 32l-33 0c-14.3 0-26.8-9.5-30.8-23.2l-7.1-24.8-82.3 0-7.1 24.8C235.8 502.5 223.3 512 209 512l-33 0c-17.7 0-32-14.3-32-32l0-29.3C87.5 426 48 369.6 48 304zm376 16a24 24 0 1 0 0-48 24 24 0 1 0 0 48z",
            }
        }
    }
}
