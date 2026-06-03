// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AustralSignProps {
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

pub fn AustralSign(props: AustralSignProps) -> Element {
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
        d: "M266.5 240l-85 0 42.5-106.4 42.5 106.4zm68.9 0L262.9 58.3C256.5 42.4 241.1 32 224 32s-32.5 10.4-38.9 26.3L112.6 240 32 240c-13.3 0-24 10.7-24 24s10.7 24 24 24l61.4 0-19.2 48-42.3 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l23.1 0-20.8 52.1c-6.6 16.4 1.4 35 17.9 41.6s35-1.4 41.6-17.9l30.3-75.9 200 0 30.3 75.9c6.6 16.4 25.2 24.4 41.6 17.9s24.4-25.2 17.9-41.6L392.9 384 416 384c13.3 0 24-10.7 24-24s-10.7-24-24-24l-42.3 0-19.2-48 61.4 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-80.6 0zm-49.7 48l19.2 48-161.6 0 19.2-48 123.3 0z",
            }
        }
    }
}
