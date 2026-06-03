// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DhlProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Dhl(props: DhlProps) -> Element {
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
        d: "M238 301.2l58.7 0 22.3-30.2-58.7 0-22.3 30.2zM0 282.9l0 6.4 81.8 0 4.7-6.4-86.5 0zM172.9 271c-8.7 0-6-3.6-4.6-5.5 2.8-3.8 7.6-10.4 10.4-14.1s2.8-5.9-2.8-5.9l-51 0-41.1 55.8 100.1 0c33.1 0 51.5-22.5 57.2-30.3l-68.2 0zm317.5-6.9l39.3-53.4-62.2 0-39.3 53.4 62.2 0zM95.3 271l-95.3 0 0 6.4 90.6 0 4.7-6.4zm111-26.6c-2.8 3.8-7.5 10.4-10.3 14.2-1.4 2-4.1 5.5 4.6 5.5l45.6 0s7.3-10 13.5-18.4c8.4-11.4 .7-35-29.2-35l-117.9 0-20.4 27.8 111.4 0c5.6 0 5.5 2.2 2.7 5.9zM0 301.2l73.1 0 4.7-6.4-77.8 0 0 6.4zm323 0l58.7 0 22.3-30.2-58.7 0c-.1 0-22.3 30.2-22.3 30.2zm222 .1l95 0 0-6.4-90.3 0-4.7 6.4zM567.3 271l-4.7 6.4 77.4 0 0-6.4-72.7 0zm-13.5 18.3l86.2 0 0-6.4-81.5 0-4.7 6.4zM389.6 210.7l-22.5 30.6-26.2 0 22.5-30.6-58.7 0-39.3 53.4 143.6 0 39.3-53.4-58.7 0zM423.1 271s-4.3 5.9-6.4 8.7c-7.4 10-.9 21.6 23.2 21.6l94.3 0 22.3-30.3-133.4 0z",
            }
        }
    }
}
