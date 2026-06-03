// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HatCowboySideProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn HatCowboySide(props: HatCowboySideProps) -> Element {
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
        d: "M640 388.3c0 16.9-7.1 32.2-18.4 43.1l-35-23.3-292.7-195.1c-36.1-24.1-78.6-36.9-122-36.9l-3.9 0c-2.7 0-5.4 0-8 .1l22.2-100c5.7-25.8 28.6-44.1 55-44.1 12.2 0 24.1 4 33.8 11.3l4.7 3.5c26.3 19.7 62.4 19.7 88.6 0l4.7-3.5c9.8-7.3 21.6-11.3 33.8-11.3 26.4 0 49.3 18.3 55 44.1l33 148.5C574.5 232.3 640 302.6 640 388.3zM171.9 224c33.9 0 67.1 10 95.4 28.9L560 448 56 448c-30.9 0-56-25.1-56-56 0-92.8 75.2-168 168-168l3.9 0z",
            }
        }
    }
}
