// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PiscesProps {
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

pub fn Pisces(props: PiscesProps) -> Element {
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
        d: "M40.4 10.4c11.9-13 32.2-13.9 45.2-2 2.9 2.7 28.3 26.7 53.6 67.8 22.3 36.2 45 86.6 51.1 147.8l67.3 0c6.1-61.1 28.8-111.6 51.1-147.8 25.3-41.1 50.7-65.1 53.6-67.8 13-11.9 33.3-11.1 45.2 2 11.9 13 11 33.3-2 45.2-1.4 1.3-22 21.1-42.4 54.2-17.9 29.1-35.4 68-41.2 114.2l94 0 3.3 .2c16.1 1.6 28.7 15.3 28.7 31.8s-12.6 30.2-28.7 31.8l-3.3 .2-94 0c5.8 46.2 23.3 85.1 41.2 114.2 20.4 33.1 41 52.9 42.4 54.2 13 11.9 13.9 32.2 2 45.2-11.9 13-32.2 13.9-45.2 2-2.9-2.7-28.3-26.7-53.6-67.8-22.3-36.2-45-86.6-51.1-147.8l-67.3 0c-6.1 61.1-28.8 111.6-51.1 147.8-25.3 41.1-50.7 65.1-53.6 67.8-13 11.9-33.3 11.1-45.2-2-11.9-13-11-33.3 2-45.2 1.4-1.3 22-21.1 42.4-54.2 17.9-29.1 35.4-68 41.2-114.2l-94 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l94 0c-5.8-46.2-23.3-85.1-41.2-114.2-20.4-33.1-41-52.9-42.4-54.2-13-11.9-13.9-32.2-2-45.2z",
            }
        }
    }
}
