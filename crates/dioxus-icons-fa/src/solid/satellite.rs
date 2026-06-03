// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SatelliteProps {
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

pub fn Satellite(props: SatelliteProps) -> Element {
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
        d: "M199 7c9.4-9.4 24.6-9.4 33.9 0l89.4 89.4 55-55c12.5-12.5 32.8-12.5 45.3 0l48 48c12.5 12.5 12.5 32.8 0 45.3l-55 55 89.4 89.4c9.4 9.4 9.4 24.6 0 33.9l-96 96c-9.4 9.4-24.6 9.4-33.9 0l-89.4-89.4-15.5 15.5c11.4 24.6 17.8 52 17.8 80.9 0 31.7-7.7 61.5-21.2 87.8-4.7 9-16.7 10.3-23.8 3.1l-96.3-96.3-60 60c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l60-60-96.3-96.3c-7.2-7.2-5.9-19.2 3.1-23.8 26.3-13.6 56.2-21.2 87.8-21.2 28.9 0 56.3 6.4 80.9 17.8L192.4 226.3 103 137c-9.4-9.4-9.4-24.6 0-33.9L199 7zm17 50.9l-62.1 62.1 72.4 72.4 62.1-62.1-72.4-72.4zM392 358.1l62.1-62.1-72.4-72.4-62.1 62.1 72.4 72.4z",
            }
        }
    }
}
