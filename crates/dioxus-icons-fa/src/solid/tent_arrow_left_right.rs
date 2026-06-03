// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TentArrowLeftRightProps {
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

pub fn TentArrowLeftRight(props: TentArrowLeftRightProps) -> Element {
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
        d: "M-.5 113c-9.4-9.4-9.4-24.6 0-33.9l72-72c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-31 31 310.1 0 54.1 0-31-31c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l72 72c9.4 9.4 9.4 24.6 0 33.9l-72 72c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l31-31-102.1 0-.1 0-262 0 31 31c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0l-72-72zM37.1 476L60.3 292.6c1.2-9.7 6.8-18.3 15.1-23.3L238.8 170c10.2-6.2 22.9-6.2 33.1-.1l165.5 99.4c8.4 5 14 13.7 15.3 23.4L475.9 476c2.4 19.1-12.5 36-31.7 36l-71.9 0c-12.1 0-23.2-6.8-28.6-17.7L275.4 357.9c-1.8-3.6-5.5-5.9-9.5-5.9-5.9 0-10.6 4.7-10.6 10.6l0 117.4c0 17.7-14.3 32-32 32L68.8 512c-19.3 0-34.2-16.9-31.7-36z",
            }
        }
    }
}
