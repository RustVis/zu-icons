// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PhoneFlipProps {
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

pub fn PhoneFlip(props: PhoneFlipProps) -> Element {
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
        d: "M351.8 25c7.8-18.8 28.4-28.9 48.1-23.5l5.5 1.5c64.6 17.6 119.8 80.2 103.7 156.4-37.1 175-174.8 312.7-349.8 349.8-76.3 16.2-138.8-39.1-156.4-103.7l-1.5-5.5c-5.4-19.7 4.7-40.3 23.5-48.1l97.3-40.5c16.5-6.9 35.6-2.1 47 11.8l38.6 47.2c70.3-34.9 126.8-93.3 159.3-164.9l-44.1-36.1c-13.9-11.3-18.6-30.4-11.8-47L351.8 25z",
            }
        }
    }
}
