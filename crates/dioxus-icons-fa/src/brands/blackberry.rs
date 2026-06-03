// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BlackberryProps {
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

pub fn Blackberry(props: BlackberryProps) -> Element {
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
        d: "M166 116.9c0 23.4-16.4 49.1-72.5 49.1l-70.1 0 21-88.8 67.8 0c42.1 0 53.8 23.3 53.8 39.7zM292.2 77.2l-67.8 0-18.7 88.8 70.1 0c53.8 0 70.1-25.7 70.1-49.1 .1-16.4-11.6-39.7-53.7-39.7zM88.8 208.1l-67.8 0-21 88.8 70.1 0c56.1 0 72.5-23.4 72.5-49.1 0-16.3-11.7-39.7-53.8-39.7zm180.1 0l-67.8 0-18.7 88.8 70.1 0c53.8 0 70.1-23.4 70.1-49.1 0-16.3-11.7-39.7-53.7-39.7zm189.3-53.8l-67.8 0-18.7 88.8 70.1 0c53.8 0 70.1-23.4 70.1-49.1 .1-16.3-11.6-39.7-53.7-39.7zm-28 137.9l-67.8 0-18.7 88.8 70.1 0c56.1 0 70.1-23.4 70.1-49.1 0-16.3-11.6-39.7-53.7-39.7zM240.8 346l-67.8 0-18.7 88.8 70.1 0c56.1 0 70.1-25.7 70.1-49.1 .1-16.3-11.6-39.7-53.7-39.7z",
            }
        }
    }
}
