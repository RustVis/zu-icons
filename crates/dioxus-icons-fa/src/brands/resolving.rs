// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ResolvingProps {
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

pub fn Resolving(props: ResolvingProps) -> Element {
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
        d: "M289.2 278.2c46-13.3 49.6-23.5 44-43.4L322 195.5c-6.1-20.9-18.4-28.1-71.1-12.8l-188.2 54.1 28.6 98.6 197.9-57.2zM256.5 8C139.4 8 41.2 88.7 15.2 197.5l221.9-63.9c34.8-10.2 54.2-11.7 79.3-8.2 36.3 6.1 52.7 25 61.4 55.2l10.7 37.8c8.2 28.1 1 50.6-23.5 73.6-19.4 17.4-31.2 24.5-61.4 33.2l-92.6 26.6 220.4 27.1 9.7 34.2-48.1 13.3-286.8-37.3 23 80.2C166 491.3 209.5 504 255.5 504 392.5 504 504 392.6 504 255.7 505 119.4 393.5 8 256.5 8zM46.3 388.6L8 256.8c0 48.5 14.3 93.4 38.3 131.8z",
            }
        }
    }
}
