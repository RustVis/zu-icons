// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SquareDeskproProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn SquareDeskpro(props: SquareDeskproProps) -> Element {
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
        d: "M408.8 32C430.4 32 448 49.6 448 71.2l0 369.6c0 21.6-17.6 39.2-39.2 39.2L39.2 480C17.6 480 0 462.4 0 440.8L0 71.2C0 49.6 17.6 32 39.2 32l369.6 0zM110.9 274.1l0 114.4 89.1-.1c20.6 0 39.4-3.2 56.3-9.5s31.3-15.4 43.2-27c12.2-11.9 21.6-26 28.2-42.1 4.6-11.1 7.7-23 9.2-35.7l-43.8 0c-1.1 6.4-2.8 12.4-5.1 18.2-4.2 10.9-10.3 20.3-18.2 28.2-7.9 7.7-17.4 13.7-28.6 17.9-11.1 4.3-23.5 6.4-37.3 6.4l-50 0 0-70.6-43 .1zm0-151.4l0 114.4 43-.1 0-70.7 50 0c13.7 0 26.2 2.1 37.3 6.4 11.1 4.3 20.6 10.3 28.6 18.3 7.9 7.7 14 17.1 18.2 28.2 2.3 5.6 4 11.5 5.1 17.8l43.9 .1c-1.4-12.6-4.4-24.4-8.9-35.3-6.6-16.4-16-30.5-28.1-42.2-12.1-11.9-26.7-21.1-43.6-27.5-16.9-6.4-35.7-9.5-56.3-9.5l-89 .1z",
            }
        }
    }
}
