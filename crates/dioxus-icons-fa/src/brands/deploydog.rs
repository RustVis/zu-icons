// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DeploydogProps {
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

pub fn Deploydog(props: DeploydogProps) -> Element {
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
        d: "M382.2 136l51.7 0 0 239.6-51.7 0 0-20.7c-19.8 24.8-52.8 24.1-73.8 14.7-26.2-11.7-44.3-38.1-44.3-71.8 0-29.8 14.8-57.9 43.3-70.8 20.2-9.1 52.7-10.6 74.8 12.9l0-103.9zM317.5 297.8a33.2 33.2 0 1 0 66.4 1 33.2 33.2 0 1 0 -66.4-1zM188.5 136l51.7 0 0 239.6-51.7 0 0-20.7c-19.8 24.8-52.8 24.1-73.8 14.7-26.2-11.7-44.3-38.1-44.3-71.8 0-29.8 14.8-57.9 43.3-70.8 20.2-9.1 52.7-10.6 74.8 12.9l0-103.9zM123.8 297.8a33.2 33.2 0 1 0 66.4 1 33.2 33.2 0 1 0 -66.4-1zM448 96c17.5 0 32 14.4 32 32l0 256c0 17.5-14.4 32-32 32L64 416c-17.5 0-32-14.4-32-32l0-256c0-17.5 14.4-32 32-32l384 0zm0-32L64 64C28.8 64 0 92.8 0 128L0 384c0 35.2 28.8 64 64 64l384 0c35.2 0 64-28.8 64-64l0-256c0-35.2-28.8-64-64-64z",
            }
        }
    }
}
