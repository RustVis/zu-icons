// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FileMedicalAltProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn FileMedicalAlt(props: FileMedicalAltProps) -> Element {
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
        d: "M0 64C0 28.7 28.7 0 64 0L213.5 0c17 0 33.3 6.7 45.3 18.7L365.3 125.3c12 12 18.7 28.3 18.7 45.3L384 448c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64l0-96 60.3 0 48.8 62.7c5.1 6.6 13.3 10 21.6 9.1s15.5-6.1 19.1-13.6l42.9-91.2 9.9 19.8c4.1 8.1 12.4 13.3 21.5 13.3l72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-57.2 0-25.4-50.7c-4.1-8.2-12.6-13.4-21.8-13.3s-17.5 5.5-21.4 13.8l-47.3 100.6-32-41.1C86.4 307.4 79.4 304 72 304L0 304 0 64zm208-5.5l0 93.5c0 13.3 10.7 24 24 24L325.5 176 208 58.5z",
            }
        }
    }
}
