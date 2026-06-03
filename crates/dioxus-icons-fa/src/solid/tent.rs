// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TentProps {
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

pub fn Tent(props: TentProps) -> Element {
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
        d: "M26.9 206.9L3.7 444.9C1.8 463.7 16.6 480 35.5 480l172.7 0c26.5 0 48-21.5 48-48l0-129c0-8.3 6.7-15 15-15 5.5 0 10.6 3 13.2 7.9l86.1 159c8.4 15.5 24.6 25.1 42.2 25.1l64.1 0c18.9 0 33.7-16.3 31.8-35.1L485.6 207.1c-1.9-19.6-12.8-37.3-29.5-47.8L280.9 48.3c-15.7-10-35.8-9.9-51.5 .1L56.1 159.2c-16.5 10.6-27.3 28.2-29.2 47.7z",
            }
        }
    }
}
