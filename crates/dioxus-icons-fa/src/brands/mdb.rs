// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MdbProps {
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

pub fn Mdb(props: MdbProps) -> Element {
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
        d: "M17.4 160.4l-10.4 191.6 43.9 0 5.6-79.8 27.9 79.8 44.7 0 25.5-77.4 4.8 77.4 45.5 0-12.8-191.6-45.5 0-40.7 117.3-42.3-117.3-46.3 0zm281 0l-47.9 0 0 191.6 47.9 0s95 .8 94.2-95.8c-.8-94.2-94.2-95.8-94.2-95.8l0 0zm-1.2 146.5l0-102.1s46 4.3 46.8 50.6-46.8 51.5-46.8 51.5l0 0zm238.3-74.2c7.2-11.4 10-25 8-38.3-5.3-35.8-55.1-34.3-55.1-34.3l-51.9 0 0 191.6 45.5 0s87 4.8 87-63.8c0-43.1-33.5-55.1-33.5-55.1l0 0zm-51.9-31.9s13.6-1.6 16 9.6c1.4 6.7-4 12-4 12l-12 0 0-21.6zm-.1 109.5l.1-24.9 0-18.2 .1 0s41.6-4.7 41.2 22.4c-.3 25.7-41.3 20.7-41.3 20.7l0 0z",
            }
        }
    }
}
