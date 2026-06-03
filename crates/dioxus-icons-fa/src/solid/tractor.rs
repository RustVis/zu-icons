// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TractorProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Tractor(props: TractorProps) -> Element {
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
        d: "M160 96l0 96 133.4 0-57.6-96-75.8 0zM96 223L96 64c0-17.7 14.3-32 32-32l107.8 0c22.5 0 43.3 11.8 54.9 31.1l77.4 128.9 64 0 0-72c0-13.3 10.7-24 24-24s24 10.7 24 24l0 72 48 0c26.5 0 48 21.5 48 48l0 41.5c0 14.2-6.3 27.8-17.3 36.9l-35 29.2c26.5 15.2 44.3 43.7 44.3 76.4 0 48.6-39.4 88-88 88s-88-39.4-88-88c0-14.4 3.5-28 9.6-40l-101.2 0c-3 13.4-7.9 26-14.4 37.7 7.7 9.4 7.2 23.4-1.6 32.2l-22.6 22.6c-8.8 8.8-22.7 9.3-32.2 1.6-9.3 5.2-19.3 9.3-29.8 12.3-1.2 12.1-11.4 21.6-23.9 21.6l-32 0c-12.4 0-22.7-9.5-23.9-21.6-10.5-3-20.4-7.2-29.8-12.3-9.4 7.7-23.4 7.2-32.2-1.6L35.5 453.8c-8.8-8.8-9.3-22.7-1.6-32.2-5.2-9.3-9.3-19.3-12.3-29.8-12.1-1.2-21.6-11.4-21.6-23.9l0-32c0-12.4 9.5-22.7 21.6-23.9 3-10.5 7.2-20.4 12.3-29.8-7.7-9.4-7.2-23.4 1.6-32.2l22.6-22.6c8.8-8.8 22.7-9.3 32.2-1.6 1.9-1 3.7-2 5.7-3zm64 65a64 64 0 1 0 0 128 64 64 0 1 0 0-128zM440 424a40 40 0 1 0 80 0 40 40 0 1 0 -80 0z",
            }
        }
    }
}
