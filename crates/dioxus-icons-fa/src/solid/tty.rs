// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TtyProps {
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

pub fn Tty(props: TtyProps) -> Element {
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
        d: "M450.2 266.8c15.8 6.5 34.1 .3 42.6-14.6l2.4-4.2c27.9-48.9 23.6-118.8-31.3-154.5-126-82-289.6-82-415.6 0-54.9 35.7-59.3 105.7-31.3 154.5l2.4 4.2c8.5 14.9 26.7 21.1 42.6 14.6l81.9-33.7c13.9-5.7 22.4-19.9 20.9-34.9l-5.1-51c62.5-21 130.8-19.9 192.6 3.3l-4.8 47.7c-1.5 15 7 29.2 20.9 34.9l81.9 33.7zM32 352a32 32 0 1 0 64 0 32 32 0 1 0 -64 0zm96 0a32 32 0 1 0 64 0 32 32 0 1 0 -64 0zM64 416a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm352 32a32 32 0 1 0 64 0 32 32 0 1 0 -64 0zM256 320a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm64 32a32 32 0 1 0 64 0 32 32 0 1 0 -64 0zm128-32a32 32 0 1 0 0 64 32 32 0 1 0 0-64zM128 448c0 17.7 14.3 32 32 32l192 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-192 0c-17.7 0-32 14.3-32 32z",
            }
        }
    }
}
