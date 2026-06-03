// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PlaneArrivalProps {
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

pub fn PlaneArrival(props: PlaneArrivalProps) -> Element {
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
        d: "M386.6 193.1L265.9 3.7c-4.8-7.6-12.6-12.8-21.4-14.3l-43.1-7.6c-10.8-1.9-20.2 7.3-18.6 18.1l24 161.6-105-18.5-33.8-61.8C64.5 74.7 58.2 70.2 51.1 69L34 66c-9.8-1.7-18.8 5.9-18.8 15.8l.6 106.3c.2 30.9 22.4 57.3 52.9 62.7l13.5 2.4 0 0 417.6 73.6c30.5 5.4 59.5-15 64.9-45.4s-15-59.5-45.4-64.9L386.6 193.1zM224 384a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm131.2-15.3a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM32 448c-17.7 0-32 14.3-32 32s14.3 32 32 32l512 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 448z",
            }
        }
    }
}
