// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HandBackFistProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn HandBackFist(props: HandBackFistProps) -> Element {
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
        d: "M7.4 253.6C2.6 245.9 0 237.1 0 228l0-36c0-26.5 21.5-48 48-48l16 0 0-80c0-26.5 21.5-48 48-48 17.3 0 32.4 9.1 40.9 22.8 4.3-22.1 23.8-38.8 47.1-38.8 23.4 0 42.9 16.8 47.1 38.9 7.3-4.4 15.8-6.9 24.9-6.9 22.1 0 40.8 15 46.3 35.4 5.5-2.2 11.4-3.4 17.7-3.4 26.5 0 48 21.5 48 48l0 96.9c0 9.9-2.3 19.7-6.8 28.6l-39.6 79.1c-10.8 21.7-33 35.4-57.2 35.4L96 352c-16.5 0-31.8-8.4-40.6-22.4l-48-76zM32 480l0-48c0-17.7 14.3-32 32-32l256 0c17.7 0 32 14.3 32 32l0 48c0 17.7-14.3 32-32 32L64 512c-17.7 0-32-14.3-32-32z",
            }
        }
    }
}
