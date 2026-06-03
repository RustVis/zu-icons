// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThemecoProps {
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

pub fn Themeco(props: ThemecoProps) -> Element {
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
        d: "M202.9 8.4c9.9-5.7 26-5.8 36-.2L430 115.9c10 5.6 18 19.4 18 30.9L448 364c0 11.4-8.1 25.3-18 31L238.8 503.7c-9.9 5.7-26 5.6-35.8-.2L17.9 395.1C8 389.3 0 375.4 0 364L0 146.7c0-11.4 8-25.4 17.9-31.1L202.9 8.4zM125.5 208.3c-15.9 0-31.9 .1-47.8 .1l0 101.4 19.1 0 0-29.8 28.7 0c49.7 0 49.6-71.7 0-71.7zM265.6 308.6l-30.7-34.6c37-7.5 34.8-65.2-10.9-65.5-16.1 0-32.2-.1-48.3-.1l0 101.6 19.1 0 0-33.9 18.4 0 29.6 33.9 22.8 0 0-1.3zm-41.6-82.3c23.3 0 23.3 32.5 0 32.5l-29.1 0 0-32.5 29.1 0zm-95.6-1.6c21.2 0 21.1 38.9 0 38.9l-32.3 0 0-38.8 32.3 0zm192.6-18.2c-68.5 0-71 105.8 0 105.8 69.5 0 69.4-105.8 0-105.8zm0 17.4c44.1 0 44.8 70.9 0 70.9s-44.4-70.9 0-70.9z",
            }
        }
    }
}
