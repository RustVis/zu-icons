// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct NonBinaryProps {
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

pub fn NonBinary(props: NonBinaryProps) -> Element {
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
        d: "M192 544c-97.2 0-176-78.8-176-176 0-86.3 62.1-158 144-173l0-47.2-49.7 24.8-3 1.3c-15.2 5.7-32.5-.8-39.9-15.7-7.4-14.8-2.2-32.6 11.5-41.3l2.8-1.6 38.8-19.4-38.8-19.4c-15.8-7.9-22.2-27.1-14.3-42.9 7.4-14.8 24.8-21.4 40-15.6l3 1.3 49.7 24.8 0-44.2c0-17.7 14.3-32 32-32s32 14.3 32 32l0 44.2 49.7-24.8 3-1.3c15.2-5.8 32.5 .8 39.9 15.6s2.2 32.7-11.5 41.3l-2.8 1.6-38.7 19.4 38.7 19.3c15.8 7.9 22.2 27.1 14.3 42.9-7.4 14.8-24.7 21.4-39.9 15.6l-3-1.3-49.7-24.8 0 47.2c81.9 15.1 144 86.8 144 173 0 97.2-78.8 176-176 176zm0-64a112 112 0 1 0 0-224 112 112 0 1 0 0 224z",
            }
        }
    }
}
