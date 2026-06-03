// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ArrowsToEyeProps {
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

pub fn ArrowsToEye(props: ArrowsToEyeProps) -> Element {
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
        d: "M176 56c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 38.1-63-63c-9.4-9.4-24.6-9.4-33.9 0S21.7 55.6 31 65l63 63-38.1 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l96 0c13.3 0 24-10.7 24-24l0-96zm0 400l0-96c0-13.3-10.7-24-24-24l-96 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l38.1 0-63 63c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l63-63 0 38.1c0 13.3 10.7 24 24 24s24-10.7 24-24zm312 24c13.3 0 24-10.7 24-24l0-38.1 63 63c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-63-63 38.1 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-96 0c-13.3 0-24 10.7-24 24l0 96c0 13.3 10.7 24 24 24zM464 56l0 96c0 13.3 10.7 24 24 24l96 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-38.1 0 63-63c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-63 63 0-38.1c0-13.3-10.7-24-24-24s-24 10.7-24 24zM320 120c-57.3 0-99.4 34-125.6 64.3-20 23.1-32.4 45.8-37.6 56.1-2.5 5-4.9 9.7-4.9 15.6s2.3 10.6 4.9 15.6c5.2 10.3 17.6 33 37.6 56.1 26.2 30.3 68.2 64.3 125.6 64.3s99.4-34 125.6-64.3c20-23.1 32.4-45.8 37.6-56.1 2.5-5 4.9-9.7 4.9-15.6s-2.3-10.6-4.9-15.6c-5.2-10.3-17.6-33-37.6-56.1-26.2-30.3-68.2-64.3-125.6-64.3zM256 256a64 64 0 1 1 128 0 64 64 0 1 1 -128 0z",
            }
        }
    }
}
