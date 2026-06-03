// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CarBurstProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn CarBurst(props: CarBurstProps) -> Element {
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
        d: "M232 16.1l0-48c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 48c0 13.3 10.7 24 24 24s24-10.7 24-24zM32 168.1l48 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-48 0c-13.3 0-24 10.7-24 24s10.7 24 24 24zM281.5 70.6c9.4 9.4 24.6 9.4 33.9 0l33.9-33.9c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0L281.5 36.6c-9.4 9.4-9.4 24.6 0 33.9zm-181 215l33.9-33.9c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0L66.6 251.6c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0zM66.6 2.7c-9.4 9.4-9.4 24.6 0 33.9l33.9 33.9c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9L100.5 2.7C91.1-6.7 76-6.7 66.6 2.7zM352.9 175.4L505 216.2c6.4 1.7 11.1 7.3 11.8 13.9l7.2 74-231.5-62 43.2-60.5c3.9-5.4 10.7-7.9 17.2-6.2zM223.6 228.5l-2.1 2.9c-21.7 5.5-39.9 22.3-46.1 45.5-4.1 15.5-12.4 46.4-24.8 92.7l-8.3 30.9c-4.6 17.1 5.6 34.6 22.6 39.2l15.5 4.1c17.1 4.6 34.6-5.6 39.2-22.6l8.3-30.9 278.2 74.5-8.3 30.9c-4.6 17.1 5.6 34.6 22.6 39.2l15.5 4.1c17.1 4.6 34.6-5.6 39.2-22.6 4.1-15.5 12.4-46.4 24.8-92.7l8.3-30.9c6.2-23.2-1.1-46.8-17.2-62.5l-.3-3.6-10-103c-3.2-33.2-26.7-60.9-58.9-69.5L369.5 113.6c-32.2-8.6-66.4 3.6-85.8 30.8l-60.2 84.2zm48.7 57.8a32 32 0 1 1 -16.6 61.8 32 32 0 1 1 16.6-61.8zm208.1 88.9a32 32 0 1 1 61.8 16.6 32 32 0 1 1 -61.8-16.6z",
            }
        }
    }
}
