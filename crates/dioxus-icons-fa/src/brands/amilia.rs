// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AmiliaProps {
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

pub fn Amilia(props: AmiliaProps) -> Element {
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
        d: "M240.2 32c-61.9 0-131.5 16.9-184.2 55.4-5.1 3.1-9.1 9.2-7.2 19.4 1.1 5.1 5.1 27.4 10.2 39.6 4.1 10.2 14.2 10.2 20.3 6.1 32.5-22.3 96.5-47.7 152.3-47.7 57.9 0 58.9 28.4 58.9 73.1l0 38.5C203.1 227.7 78.3 251 46.8 264.2 11.3 280.5 16.4 357.7 16.4 376S31.6 480 141.3 480c47.8 0 113.7-20.7 153.3-42.1l0 25.4c0 3 2.1 8.2 6.1 9.1 3.1 1 50.7 2 59.9 2s62.5 .3 66.5-.7c4.1-1 5.1-6.1 5.1-9.1l0-296.6c-.1-80.3-57.9-136-192-136zm50.2 348c-21.4 13.2-48.7 24.4-79.1 24.4-52.8 0-58.9-33.5-59-44.7 0-12.2-3-42.7 18.3-52.9 24.3-13.2 75.1-29.4 119.8-33.5l0 106.7z",
            }
        }
    }
}
