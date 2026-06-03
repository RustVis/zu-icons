// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HotTubPersonProps {
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

pub fn HotTubPerson(props: HotTubPersonProps) -> Element {
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
        d: "M240 40c0 13.6 5.8 26.5 15.8 35.6l26.5 23.8c24 21.6 37.7 52.3 37.7 84.6 0 13.3-10.7 24-24 24s-24-10.7-24-24c0-18.7-7.9-36.4-21.8-48.9l-26.5-23.8C203.5 93.1 192 67.2 192 40 192 26.7 202.7 16 216 16s24 10.7 24 24zM0 336l0-80c0-35.3 28.7-64 64-64l19.7 0c8.1 0 16.2 1.6 23.8 4.6l137.1 54.8c7.6 3 15.6 4.6 23.8 4.6L384 256c35.3 0 64 28.7 64 64l0 128c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 336zm96-16c-13.3 0-24 10.7-24 24l0 80c0 13.3 10.7 24 24 24s24-10.7 24-24l0-80c0-13.3-10.7-24-24-24zm152 24c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 80c0 13.3 10.7 24 24 24s24-10.7 24-24l0-80zm104-24c-13.3 0-24 10.7-24 24l0 80c0 13.3 10.7 24 24 24s24-10.7 24-24l0-80c0-13.3-10.7-24-24-24zM328 16c13.3 0 24 10.7 24 24 0 13.6 5.8 26.5 15.8 35.6l26.5 23.8c24 21.6 37.7 52.3 37.7 84.6 0 13.3-10.7 24-24 24s-24-10.7-24-24c0-18.7-7.9-36.4-21.8-48.9l-26.5-23.8C315.5 93.1 304 67.2 304 40 304 26.7 314.7 16 328 16zM64 40a56 56 0 1 1 0 112 56 56 0 1 1 0-112z",
            }
        }
    }
}
