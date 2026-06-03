// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct KiwiBirdProps {
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

pub fn KiwiBird(props: KiwiBirdProps) -> Element {
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
        d: "M291.2 388.4c31.2-18.8 64.7-36.4 101.1-36.4l55.7 0c4.6 0 9.1-.2 13.6-.7l85.3 121.9c4 5.7 11.3 8.2 17.9 6.1S576 471 576 464l0-240c0-70.7-57.3-128-128-128l-55.7 0C355.9 96 322.4 78.4 291.2 59.6 262.3 42.1 228.3 32 192 32 86 32 0 118 0 224 0 295.1 38.6 357.1 96 390.3L96 456c0 13.3 10.7 24 24 24s24-10.7 24-24l0-46c15.3 3.9 31.4 6 48 6 5.4 0 10.7-.2 16-.7l0 40.7c0 13.3 10.7 24 24 24s24-10.7 24-24l0-50.9c12.4-4.4 24.2-10 35.2-16.7zM448 200a24 24 0 1 1 0 48 24 24 0 1 1 0-48z",
            }
        }
    }
}
