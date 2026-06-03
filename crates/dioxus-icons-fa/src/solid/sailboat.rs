// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SailboatProps {
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

pub fn Sailboat(props: SailboatProps) -> Element {
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
        d: "M240 35.9l0 348.1-224 0c-8.8 0-16 7.2-16 16 0 61.9 50.1 112 112 112l288 0c61.9 0 112-50.1 112-112 0-8.8-7.2-16-16-16l-208 0 0-48 161.7 0c12.9 0 20.5-14.4 13.2-25L269.2 26.9c-8.9-13-29.2-6.7-29.2 9zM192 320l0-184.2c0-16.1-21-22.1-29.6-8.5L47.3 311.5c-6.7 10.7 1 24.5 13.6 24.5L176 336c8.8 0 16-7.2 16-16z",
            }
        }
    }
}
