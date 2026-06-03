// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct NairaSignProps {
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

pub fn NairaSign(props: NairaSignProps) -> Element {
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
        d: "M122.6 46.3c-7.8-11.7-22.4-17-35.9-12.9S64 49.9 64 64l0 192-32 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l32 0 0 144c0 17.7 14.3 32 32 32s32-14.3 32-32l0-144 89.5 0 107.8 161.8c7.8 11.7 22.4 17 35.9 12.9S384 462.1 384 448l0-144 32 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-32 0 0-192c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 192-57.5 0-139.8-209.7zM294.5 304l25.5 0 0 38.3-25.5-38.3zM185.5 256l-57.5 0 0-86.3 57.5 86.3z",
            }
        }
    }
}
