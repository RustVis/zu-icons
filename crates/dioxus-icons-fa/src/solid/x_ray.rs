// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct XRayProps {
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

pub fn XRay(props: XRayProps) -> Element {
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
        d: "M0 64C0 46.3 14.3 32 32 32l448 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l0 320c17.7 0 32 14.3 32 32s-14.3 32-32 32L32 480c-17.7 0-32-14.3-32-32s14.3-32 32-32L32 96C14.3 96 0 81.7 0 64zm280 56c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 16-64 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l64 0 0 48-80 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l80 0 0 48-64 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l64 0 0 16c0 13.3 10.7 24 24 24s24-10.7 24-24l0-16 64 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-64 0 0-48 80 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-80 0 0-48 64 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-64 0 0-16z",
            }
        }
    }
}
