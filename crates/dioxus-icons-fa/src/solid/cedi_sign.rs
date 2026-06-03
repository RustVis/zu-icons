// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CediSignProps {
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

pub fn CediSign(props: CediSignProps) -> Element {
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
        d: "M232 32c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 26.5C88.8 73.9 16 156.4 16 256S88.8 438.1 184 453.5l0 26.5c0 13.3 10.7 24 24 24s24-10.7 24-24l0-24.6c46.8-3.7 89.1-23.6 121.3-53.9 12.9-12.1 13.4-32.4 1.3-45.2s-32.4-13.4-45.2-1.3c-20.7 19.6-47.6 32.7-77.3 36.2l0-270.1c29.8 3.5 56.6 16.6 77.3 36.2 12.9 12.1 33.1 11.5 45.2-1.3s11.5-33.1-1.3-45.2C321.1 80.2 278.8 60.3 232 56.6L232 32zm-48 91.8l0 264.4C124.3 373.8 80 320.1 80 256s44.3-117.8 104-132.2z",
            }
        }
    }
}
