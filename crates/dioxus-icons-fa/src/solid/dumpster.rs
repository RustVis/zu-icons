// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DumpsterProps {
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

pub fn Dumpster(props: DumpsterProps) -> Element {
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
        d: "M132.3 64L106.7 192 24 192c-13.3 0-24-10.7-24-24l0-4.1c0-2.6 .4-5.1 1.2-7.6L26.5 80.4C29.8 70.6 39 64 49.3 64l83 0zm23.4 128l25.6-128 82.7 0 0 128-108.3 0zM312 64l82.7 0 25.6 128-108.3 0 0-128zm131.7 0l83 0c10.3 0 19.5 6.6 22.8 16.4l25.3 75.9c.8 2.4 1.2 5 1.2 7.6l0 4.1c0 13.3-10.7 24-24 24l-82.7 0-25.6-128zM25.7 240l524.7 0c-.4 2.1-13.3 73.4-38.9 213.7-3 16.3-17.9 27.6-34.4 26.1S448 464.6 448 448l0-16-320 0 0 16c0 16.6-12.6 30.4-29.1 31.9S67.5 470 64.5 453.7C39 313.3 26 242.1 25.7 240z",
            }
        }
    }
}
