// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PooBoltProps {
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

pub fn PooBolt(props: PooBoltProps) -> Element {
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
        d: "M268.9-31.8c-5.5-.7-11 1.4-14.5 5.7s-4.6 10.1-2.8 15.3c2.8 8.2 4.3 16.9 4.3 26.1 0 21.7-8.5 37.2-21.9 47.6-13.9 10.8-34.1 17-58.1 17l-24 0c-48.6 0-88 39.4-88 88 0 14.8 3.7 28.8 10.2 41.1-42 6.6-74.2 43-74.2 86.9 0 46.6 36.2 84.7 81.9 87.8 3.1-12.4 9.9-23.8 19.6-32.5L242.7 225c12.2-10.9 28-17 44.4-17 44.6 0 76.5 43 63.7 85.7l-12.7 42.4c28.8 1.2 52.7 21.1 59.8 47.9l26.2 0c48.6 0 88-39.4 88-88 0-43.9-32.1-80.3-74.2-86.9 6.5-12.3 10.2-26.2 10.2-41.1 0-48.6-39.4-88-88-88l-9.4 0c.9-5.4 1.4-10.9 1.4-16.6 0-48.7-36.1-88.9-83.1-95.2zM144.6 416l61.8 0-31.2 104.1c-3.6 11.9 5.3 23.9 17.8 23.9 4.6 0 9-1.7 12.4-4.7L346.5 412.9c3.5-3.1 5.5-7.6 5.5-12.4 0-9.2-7.4-16.6-16.6-16.6l-61.8 0 31.2-104.1c3.6-11.9-5.3-23.9-17.8-23.9-4.6 0-9 1.7-12.4 4.7L133.5 387.1c-3.5 3.1-5.5 7.6-5.5 12.4 0 9.2 7.4 16.6 16.6 16.6z",
            }
        }
    }
}
