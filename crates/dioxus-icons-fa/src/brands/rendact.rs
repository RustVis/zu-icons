// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RendactProps {
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

pub fn Rendact(props: RendactProps) -> Element {
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
        d: "M256 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zM427.3 166.6c-15.2 34.5-30.4 69-45.6 103.5-2.4 5.5-6.9 8.2-13 8.2-23 0-46 .1-69 0-5.1 0-8.2 1.9-10.3 6.7-10.2 23.6-20.6 47-31 70.5-1.5 3.5-4.1 5.3-7.9 5.3-45.9 0-91.9 0-137.8 0-3.1 0-5.6-1.1-7.7-3.4-11.2-12.3-22.5-24.6-33.7-36.9-2.7-3-2.8-6.2-1.2-9.7 8.7-19.5 17.3-39.1 25.9-58.7 12.9-29.4 25.9-58.7 38.7-88.1 1.7-3.9 4.3-5.7 8.5-5.7 14.2 .1 28.5 0 42.7 0 6.2 0 9.2 4.8 6.7 10.6-13.6 30.8-27.2 61.6-40.7 92.3-5.7 13-11.4 26-17.1 39-3.9 9 7.1 12 11 5.6 .2-.4-1.4 4.2 30-67.7 1.4-3.1 3.4-4.4 6.8-4.4 15.2 .1 30.4 0 45.6 0 5.6 0 7.9 3.6 5.7 8.7-8.3 19-16.7 37.9-25 56.9-5 11.4 8.1 12.5 11.3 5.3 0-.1 27.9-63.3 32.2-73.2 2-4.6 5.4-6.5 10.3-6.5 26.4 .1 52.9 0 79.3 0 12.4 0 13.9-13.6 3.9-13.6-25.3 0-50.5 0-75.8 0-6.3 0-7.8-2.5-5.3-8.3 5.8-13.1 11.6-26.1 17.3-39.2 1.7-4 4.5-5.8 8.8-5.8 23.1 .1 26 0 130.8 0 6.1 0 8 2.8 5.6 8.3z",
            }
        }
    }
}
