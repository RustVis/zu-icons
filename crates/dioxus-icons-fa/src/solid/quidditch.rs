// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct QuidditchProps {
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

pub fn Quidditch(props: QuidditchProps) -> Element {
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
        d: "M496 544a80 80 0 1 1 0-160 80 80 0 1 1 0 160zM106.8 277.2c30.6-30.6 73.1-45.9 115.8-42.2L341 353.3c3.7 42.8-11.6 85.2-42.2 115.9-27.4 27.4-64.6 42.8-103.3 42.8L22.1 512c-12.2 0-22.1-9.9-22.1-22.1 0-6.3 2.7-12.3 7.3-16.5L133.7 359.7c4.2-3.7-.4-10.4-5.4-7.9L77.2 377.4c-6.1 3-13.2-1.4-13.2-8.2 0-31.5 12.5-61.8 34.8-84l8-8zm417-270c12.6-10.3 31.1-9.5 42.8 2.2s12.4 30.2 2.2 42.8l-2.2 2.4-192 192 34.8 34.7c4.2 4.2 6.6 10 6.6 16 0 12.5-10.1 22.6-22.6 22.6l-29.1 0-108.3-108.3 0-29.1c0-12.5 10.1-22.6 22.6-22.6 6 0 11.8 2.4 16 6.6l34.8 34.7 192-192 2.4-2.2z",
            }
        }
    }
}
