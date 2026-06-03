// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PersonPregnantProps {
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

pub fn PersonPregnant(props: PersonPregnantProps) -> Element {
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
        d: "M192 80a56 56 0 1 0 0-112 56 56 0 1 0 0 112zm80 321.2c8.6-4.5 16.8-10 24.3-16.5l4-3.4c22.6-19.4 35.7-47.7 35.7-77.6 0-35.9-18.8-69.1-49.6-87.6l-30.4-18.2 0-1.8c0-46.5-37.7-84.1-84.1-84.1-28.1 0-54.4 14.1-70 37.5L21.4 270.2c-9.8 14.7-5.8 34.6 8.9 44.4s34.6 5.8 44.4-8.9l29-43.5-30.5 113.5c-2.6 9.6-.6 19.9 5.5 27.8S94 416 104 416l8 0 0 96c0 17.7 14.3 32 32 32s32-14.3 32-32l0-96 32 0 0 96c0 17.7 14.3 32 32 32s32-14.3 32-32l0-110.8z",
            }
        }
    }
}
