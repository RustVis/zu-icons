// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FaucetDripProps {
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

pub fn FaucetDrip(props: FaucetDripProps) -> Element {
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
        d: "M224 32c-17.7 0-32 14.3-32 32L96 64C78.3 64 64 78.3 64 96s14.3 32 32 32l96 0 0 64-18.7 0c-8.5 0-16.6 3.4-22.6 9.4L128 224 32 224c-17.7 0-32 14.3-32 32l0 64c0 17.7 14.3 32 32 32l100.1 0c20.2 29 53.9 48 91.9 48s71.7-19 91.9-48l36.1 0c17.7 0 32 14.3 32 32s14.3 32 32 32l64 0c17.7 0 32-14.3 32-32 0-88.4-71.6-160-160-160l-32 0-22.6-22.6c-6-6-14.1-9.4-22.6-9.4l-18.7 0 0-64 96 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 0c0-17.7-14.3-32-32-32zM436.8 455.4l-18.2 42.4c-1.8 4.1-2.7 8.6-2.7 13.1l0 1.2c0 17.7 14.3 32 32 32s32-14.3 32-32l0-1.2c0-4.5-.9-8.9-2.7-13.1l-18.2-42.4c-1.9-4.5-6.3-7.4-11.2-7.4s-9.2 2.9-11.2 7.4z",
            }
        }
    }
}
