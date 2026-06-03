// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TheaterMasksProps {
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

pub fn TheaterMasks(props: TheaterMasksProps) -> Element {
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
        d: "M-5 118L23.5 279.7c14 79.5 76.3 141.8 155.8 155.8l12.7 2.2c-16.5-28.6-27.1-60.7-30.6-94.5l-24.1 4.3c-9.7 1.7-18.8-5.8-16.9-15.5 4.8-24.7 19.1-46.6 39.7-60.9l0-74.6c-1.4 .8-3 1.3-4.7 1.6l-63 11.1c-8.7 1.5-17.3-4.4-15.9-13.1 3.1-19.6 18.4-36 39.1-39.7 17.2-3 33.9 3.5 44.6 15.8l0-22.7c0-22.5 6.9-52.4 32.3-73.4 26-21.5 67.7-43.9 124.9-54.2-30.5-16.3-86.3-32-163.8-18.4-80.3 14.2-128 50.1-150.1 76.1-9 10.5-10.8 24.9-8.4 38.5zM208 138.7l0 174.8c0 80.7 50.5 152.9 126.4 180.4L362.1 504c14.1 5.1 29.6 5.1 43.7 0L433.6 494C509.5 466.4 560 394.3 560 313.5l0-174.8c0-6.9-2.1-13.8-7-18.6-22.6-22.5-78.2-56-169-56s-146.4 33.6-169 56c-4.9 4.9-7 11.7-7 18.6zm66.1 187.1c-1.4-7 7-11 12.7-6.6 26.9 20.6 60.6 32.9 97.2 32.9s70.2-12.3 97.2-32.9c5.7-4.4 14.1-.4 12.7 6.6-10.1 51.4-55.5 90.3-109.9 90.3s-99.8-38.8-109.9-90.3zm.5-101.5C281.2 205.5 299 192 320 192s38.9 13.5 45.4 32.3c2.9 8.4-4.5 15.7-13.4 15.7l-64 0c-8.8 0-16.3-7.4-13.4-15.7zM480 240l-64 0c-8.8 0-16.3-7.4-13.4-15.7 6.5-18.8 24.4-32.3 45.4-32.3s38.9 13.5 45.4 32.3c2.9 8.4-4.5 15.7-13.4 15.7z",
            }
        }
    }
}
