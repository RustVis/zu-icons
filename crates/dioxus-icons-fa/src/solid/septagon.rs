// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SeptagonProps {
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

pub fn Septagon(props: SeptagonProps) -> Element {
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
        d: "M267.4-31.5c15.9-5.5 33.5-4.6 48.8 2.7l172.4 83 6.3 3.5c14.2 9 24.5 23.3 28.3 39.9l42.6 186.5 1.2 7.2c1.6 14.3-1.7 28.8-9.4 41.1l-4.2 5.9-119.3 149.6c-12.1 15.2-30.6 24.1-50 24.1l-191.3 0c-19.5 0-37.9-8.9-50-24.1L23.5 338.4c-12.1-15.2-16.7-35.2-12.3-54.2l42.6-186.5 2-7c5.6-15.9 17.2-29 32.6-36.4l172.4-83 6.7-2.7z",
            }
        }
    }
}
