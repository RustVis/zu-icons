// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SquarespaceProps {
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

pub fn Squarespace(props: SquarespaceProps) -> Element {
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
        d: "M186.1 343.3c-9.7 9.7-9.7 25.3 0 34.9s25.3 9.6 34.9 0L378.3 221.1c19.3-19.3 50.6-19.3 69.9 0s19.3 50.6 0 69.9L294 445.1c19.3 19.3 50.5 19.3 69.8 0l0 0 119.3-119.2c38.6-38.6 38.6-101.1 0-139.7-38.6-38.6-101.2-38.6-139.7 0L186.1 343.3zM430.7 238.5c-9.7-9.7-25.3-9.7-34.9 0L238.5 395.7c-19.3 19.3-50.5 19.3-69.8 0l0 0c-9.6-9.6-25.3-9.7-34.9 0l0 0c-9.7 9.6-9.7 25.3 0 34.9l0 0c38.6 38.6 101.1 38.6 139.7 0L430.7 273.5c9.6-9.7 9.6-25.3 0-34.9zm-262 87.3L325.9 168.7c9.6-9.6 9.6-25.3 0-34.9-9.6-9.6-25.3-9.6-34.9 0L133.7 290.9c-19.3 19.3-50.6 19.3-69.9 0l0 0c-19.3-19.3-19.3-50.5 0-69.8l0 0 154.2-154.2c-19.3-19.3-50.5-19.3-69.8 0l0 0-119.2 119.3c-38.6 38.6-38.6 101.1 0 139.7 38.6 38.6 101.1 38.6 139.7 0zM81.3 273.5c9.6 9.6 25.3 9.6 34.9 0L273.5 116.3c19.3-19.3 50.6-19.3 69.8 0l0 0c9.7 9.6 25.3 9.6 34.9 0s9.6-25.3 0-34.9c-38.6-38.6-101.1-38.6-139.7 0L81.3 238.5c-9.6 9.6-9.6 25.3 0 34.9l0 0z",
            }
        }
    }
}
