// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UberProps {
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

pub fn Uber(props: UberProps) -> Element {
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
        d: "M414.1 32L33.9 32C15.2 32 0 47.2 0 65.9L0 446c0 18.8 15.2 34 33.9 34L414 480c18.7 0 33.9-15.2 33.9-33.9l0-380.2C448 47.2 432.8 32 414.1 32zM237.6 391.1C163 398.6 96.4 344.2 88.9 269.6l94.4 0 0 20.4c0 3.7 3 6.8 6.8 6.8l67.9 0c3.7 0 6.8-3 6.8-6.8l0-67.9c0-3.7-3-6.8-6.8-6.8l-67.9 0c-3.7 0-6.8 3-6.8 6.8l0 20.4-94.4 0c7-69.4 65.4-122.2 135.1-122.2s128.1 52.8 135.1 122.2c7.5 74.5-46.9 141.1-121.5 148.6z",
            }
        }
    }
}
