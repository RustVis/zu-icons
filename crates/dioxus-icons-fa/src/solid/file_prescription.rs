// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FilePrescriptionProps {
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

pub fn FilePrescription(props: FilePrescriptionProps) -> Element {
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
        d: "M0 64C0 28.7 28.7 0 64 0L213.5 0c17 0 33.3 6.7 45.3 18.7L365.3 125.3c12 12 18.7 28.3 18.7 45.3L384 448c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zm208-5.5l0 93.5c0 13.3 10.7 24 24 24L325.5 176 208 58.5zM88 192c-13.3 0-24 10.7-24 24l0 144c0 13.3 10.7 24 24 24s24-10.7 24-24l0-40 22.5 0 58.4 55-33.4 31.6c-9.6 9.1-10.1 24.3-.9 33.9s24.3 10.1 33.9 .9l35.4-33.5 35.6 33.5c9.7 9.1 24.8 8.6 33.9-1s8.6-24.8-1-33.9l-33.6-31.6 33.6-31.8c9.6-9.1 10.1-24.3 .9-33.9s-24.3-10.1-33.9-.9l-35.7 33.7-40.9-38.5c12.9-11.7 21.1-28.6 21.1-47.5 0-35.3-28.7-64-64-64l-56 0zm32 80l-8 0 0-32 32 0c8.8 0 16 7.2 16 16s-7.2 16-16 16l-24 0z",
            }
        }
    }
}
