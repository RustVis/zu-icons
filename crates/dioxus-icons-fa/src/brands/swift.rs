// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwiftProps {
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

pub fn Swift(props: SwiftProps) -> Element {
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
        d: "M448 156.1c0-4.5-.1-9-.2-13.5-.1-9.9-1-19.7-2.6-29.4-1.7-9.7-4.8-19.2-9.2-28-9-17.7-23.4-32.1-41.2-41.1-8.8-4.5-18.3-7.6-28-9.2-9.7-1.6-19.6-2.5-29.4-2.6-4.5-.1-9-.2-13.5-.2L124.1 32c-4.5 0-9 .1-13.5 .2-2.4 .1-4.9 .2-7.4 .3-7.4 .3-14.8 1.1-22.1 2.3-7.3 1.3-14.4 3.3-21.2 6.1-2.3 1-4.6 2-6.8 3.1-6.6 3.4-12.8 7.5-18.4 12.3-1.9 1.6-3.7 3.3-5.4 5-7 7-12.9 15-17.4 23.8-4.4 8.8-7.6 18.3-9.2 28-1.6 9.7-2.4 19.6-2.5 29.4-.1 4.5-.2 9-.2 13.5L0 355.9c0 4.5 .1 9 .2 13.5 .1 9.9 1 19.7 2.6 29.4 1.7 9.7 4.8 19.2 9.2 28 9 17.7 23.4 32.1 41.1 41.1 8.8 4.4 18.3 7.5 28 9.2 9.7 1.6 19.6 2.5 29.4 2.6 4.5 .1 9 .2 13.5 .2l199.8 0c4.5 0 9-.1 13.5-.2 9.9-.1 19.7-1 29.4-2.6 9.7-1.7 19.2-4.8 28-9.2 17.7-9 32.1-23.4 41.1-41.2 4.4-8.8 7.6-18.3 9.2-28 1.6-9.7 2.5-19.6 2.6-29.4 .1-4.5 .2-9 .2-13.5l0-183.8c0-5.4 0-10.7 0-16zm-69.9 241c-20-38.9-57.2-29.3-76.3-19.5-1.7 1-3.5 2-5.2 3l-.4 .2c-39.5 21-92.5 22.5-145.8-.4-43.4-18.8-80.1-50.3-105.3-90.4 12.3 9.1 25.4 16.9 39.2 23.4 56.4 26.4 113 24.5 153 0-57-43.8-104.6-101-141.1-147.2-7-8.1-13.2-16.8-18.8-25.9 43.7 40 112.7 90.2 137.5 104.1-52.6-55.5-98.9-123.9-96.7-121.7 82.8 83.4 159.2 130.6 159.2 130.6 2.9 1.6 5 2.8 6.7 4 1.6-4.1 3-8.2 4.2-12.5 13.2-48.3-1.7-103.6-35.3-149.2 76.8 46.1 122.2 133.7 103.6 207.8-.4 1.7-1 3.4-1.4 5.1 38.5 47.4 28 98.2 23.1 88.6l0 0z",
            }
        }
    }
}
