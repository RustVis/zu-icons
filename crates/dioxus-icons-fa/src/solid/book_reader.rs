// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BookReaderProps {
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

pub fn BookReader(props: BookReaderProps) -> Element {
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
        d: "M256 152a88 88 0 1 0 0-176 88 88 0 1 0 0 176zm0 298.7l0-149.3c16.3-6.8 32.9-13.7 49.7-20.7 39-16.2 80.8-24.6 123.1-24.6l19.2 0 0 160-19.2 0c-59.1 0-117.7 11.7-172.3 34.5l-.5 .2zM256 232l-25.1-10.5C184.1 202 133.9 192 83.2 192L48 192c-26.5 0-48 21.5-48 48L0 432c0 26.5 21.5 48 48 48l35.2 0c50.7 0 100.9 10 147.7 29.5l12.8 5.3c7.9 3.3 16.7 3.3 24.6 0l12.8-5.3c46.8-19.5 97-29.5 147.7-29.5l35.2 0c26.5 0 48-21.5 48-48l0-192c0-26.5-21.5-48-48-48l-35.2 0c-50.7 0-100.9 10-147.7 29.5L256 232z",
            }
        }
    }
}
