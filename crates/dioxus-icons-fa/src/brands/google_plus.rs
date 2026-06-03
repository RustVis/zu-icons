// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GooglePlusProps {
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

pub fn GooglePlus(props: GooglePlusProps) -> Element {
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
        d: "M256 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zM185.3 380c-32.9 0-64.4-13.1-87.7-36.3s-36.3-54.8-36.3-87.7 13.1-64.4 36.3-87.7 54.8-36.3 87.7-36.3c31.3 0 60.1 11 83 32.3l-33.6 32.6c-13.2-12.9-31.3-19.1-49.4-19.1-42.9 0-77.2 35.5-77.2 78.1S142.3 334 185.3 334c32.6 0 64.9-19.1 70.1-53.3l-70.1 0 0-42.6 116.9 0c1.3 6.8 1.9 13.8 1.9 20.7 0 70.8-47.5 121.2-118.8 121.2zM415.5 273.8l0 35.5-35.5 0 0-35.5-35.5 0 0-35.5 35.5 0 0-35.5 35.5 0 0 35.5 35.2 0 0 35.5-35.2 0z",
            }
        }
    }
}
