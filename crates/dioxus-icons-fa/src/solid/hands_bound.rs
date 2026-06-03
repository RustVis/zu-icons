// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HandsBoundProps {
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

pub fn HandsBound(props: HandsBoundProps) -> Element {
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
        d: "M64 32C64 14.3 49.7 0 32 0S0 14.3 0 32L0 213.9c0 14.2 5.1 27.9 14.3 38.7L99.6 352 96 352c-13.3 0-24 10.7-24 24s10.7 24 24 24l384 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-3.6 0 85.3-99.5c9.2-10.8 14.3-24.5 14.3-38.7L576 32c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 112.8-69.3 92.4c-5.7 7.6-16.1 9.6-24.2 4.8-9.7-5.7-12.1-18.7-5.1-27.5L441 180c10.8-13.5 8.9-33.3-4.4-44.5s-33-9.8-44.5 3.2l-46.7 52.5C329 209.7 320 233.4 320 258.1l0 93.9-64 0 0-93.9c0-24.6-9-48.4-25.4-66.8l-46.7-52.5c-11.5-13-31.3-14.4-44.5-3.2S124.2 166.4 135 180l27.6 34.5c7 8.8 4.7 21.8-5.1 27.5-8.1 4.8-18.6 2.7-24.2-4.8L64 144.8 64 32zm64 448l0 32 128 0 0-32 64 0 0 32 128 0 0-32 32 0c13.3 0 24-10.7 24-24s-10.7-24-24-24L96 432c-13.3 0-24 10.7-24 24s10.7 24 24 24l32 0z",
            }
        }
    }
}
