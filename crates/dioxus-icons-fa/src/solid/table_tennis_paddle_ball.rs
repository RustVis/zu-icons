// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableTennisPaddleBallProps {
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

pub fn TableTennisPaddleBall(props: TableTennisPaddleBallProps) -> Element {
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
        d: "M97 127l67.4-67.4c38.2-38.2 90-59.6 144-59.6 112.5 0 203.7 91.2 203.7 203.6 0 46.4-15.8 91.1-44.5 127-23.6-16.8-52.4-26.7-83.5-26.7-31.1 0-59.9 9.9-83.4 26.6L97 127zM240 448c0 9.7 1 19.1 2.8 28.2-19.8-5.2-38-15.5-52.7-30.2-12.2-12.2-31.9-12.2-44.1 0L96.6 495.4c-10.6 10.6-25 16.6-40 16.6-31.2 0-56.6-25.3-56.6-56.6 0-15 6-29.4 16.6-40l49.4-49.4c12.2-12.2 12.2-31.9 0-44.1-21.7-21.7-33.9-51.2-33.9-81.9 0-29.4 11.1-57.6 31.1-79L266.6 364.6C249.9 388.1 240 416.9 240 448zm144-96a96 96 0 1 1 0 192 96 96 0 1 1 0-192z",
            }
        }
    }
}
