// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SpellCheckProps {
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

pub fn SpellCheck(props: SpellCheckProps) -> Element {
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
        d: "M120 32c-48.6 0-88 39.4-88 88l0 168c0 17.7 14.3 32 32 32s32-14.3 32-32l0-64 64 0 0 64c0 17.7 14.3 32 32 32s32-14.3 32-32l0-168c0-48.6-39.4-88-88-88l-16 0zm40 128l-64 0 0-40c0-13.3 10.7-24 24-24l16 0c13.3 0 24 10.7 24 24l0 40zM304 32c-17.7 0-32 14.3-32 32l0 224c0 17.7 14.3 32 32 32l72 0c48.6 0 88-39.4 88-88 0-23.6-9.3-45-24.4-60.8 10.3-14.4 16.4-32.1 16.4-51.2 0-48.6-39.4-88-88-88l-64 0zm64 112l-32 0 0-48 32 0c13.3 0 24 10.7 24 24s-10.7 24-24 24zM336 256l0-48 40 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-40 0zm233 84c11-13.8 8.8-33.9-5-45s-33.9-8.8-45 5l-105.7 132.1-38.7-38.7c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l64 64c6.4 6.4 15.3 9.8 24.4 9.3s17.5-4.9 23.2-12L569 340z",
            }
        }
    }
}
