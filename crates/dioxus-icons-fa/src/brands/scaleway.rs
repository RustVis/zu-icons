// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ScalewayProps {
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

pub fn Scaleway(props: ScalewayProps) -> Element {
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
        d: "M243.4 117.3l-84.3 0c-18.5 2.1-32.7 17.5-33.4 36l0 121.9c0 8.2 3.2 15.3 8.5 20.7s12.5 8.6 20.6 8.6c16 0 29.2-13.2 29.2-29.2l0-77.4c0-12.1 10-22.1 22.1-22.1l38.1 0c8.2 0 15.3-3.2 20.6-8.6s8.5-12.8 8.5-21c0-16-13.9-28.9-29.9-28.9zM205.3 395.4l84.3 0c18.5-2.1 32.7-17.5 33.4-36l0-121.9c0-8.2-3.2-15.3-8.5-20.7s-12.5-8.6-20.6-8.6c-16 0-29.2 13.2-29.2 29.2l0 77.4c0 12.1-10 22.1-22.1 22.1l-38.1 0c-8.2 0-15.3 3.2-20.6 8.6s-8.5 12.8-8.5 21c.4 16 13.9 28.9 29.9 28.9zM104.3 0L257.6 0c101 0 182.5 81.6 182.1 182.9l0 242.8c-4.6 46-42 82.4-88.2 86.3L190 512C89.3 512 7.5 430 7.5 329.1L7.5 97c0-53.5 43.4-97 96.8-97zM381.7 182.9c0-68.5-55.9-124.4-124.2-124.4l-153 0C83.3 58.5 66.2 75.6 66.2 97l0 232.1c0 68.8 55.5 124.4 124.2 124.4l158.7 0c16.7-2.1 29.9-15 32.7-31.4l0-239.2z",
            }
        }
    }
}
