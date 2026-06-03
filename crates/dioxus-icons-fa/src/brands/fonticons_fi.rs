// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FonticonsFiProps {
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

pub fn FonticonsFi(props: FonticonsFiProps) -> Element {
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
        d: "M114.4 224l92.4 0-15.2 51.2-76.4 0 0 157.8c0 8-2.8 9.2 4.4 10l59.6 5.6 0 34.4-179.2 0 0-35.2 29.2-2.8c7.2-.8 9.2-3.2 9.2-10.8l0-155.8c0-3.2-4-3.2-8-3.2l-30.4 0 0-51.2 38.4 0 0-28.8c0-68 36.4-96 106-96 46.8 0 88.8 11.2 88.8 72.4L163.6 180c.4-25.6-6-31.6-22.4-31.6-25.2 0-26 13.6-26 37.6l0 32c0 3.2-4.8 6-.8 6zM384 483l-140.8 0 0-34.4 28-3.6c7.2-.8 10.4-2.4 10.4-10l0-148c0-5.6-4-9.2-9.2-10.8l-33.2-8.8 9.2-40.4 110 0 0 208c0 8-3.6 8.8 4 10l21.6 3.6 0 34.4zM354 135.8l12.4 45.6-10 10-42.8-22.8-42.8 22.8-10-10 12.4-45.6-30-36.4 4.8-10 38 0 21.2-38.4 12.8 0 21.2 38.4 38 0 4.8 13.2-30 33.2z",
            }
        }
    }
}
