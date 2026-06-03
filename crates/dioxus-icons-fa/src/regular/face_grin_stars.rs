// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FaceGrinStarsProps {
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

pub fn FaceGrinStars(props: FaceGrinStarsProps) -> Element {
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
        d: "M0 256c0-29.6 5-57.9 14.2-84.4l17.3 16.9-4.6 27c-4.2 24.4 5.6 46.2 22 59.9 9.8 105.8 98.8 188.7 207.1 188.7s197.4-82.8 207.1-188.6c16.4-13.7 26.1-35.4 22-59.9l-4.6-27 17.3-16.9c9.2 26.4 14.2 54.8 14.2 84.4 0 141.4-114.6 256-256 256S0 397.4 0 256zM256 48c-15.2 0-30 1.6-44.3 4.7L201.4 31.8C197 23 191.1 15.8 184.2 10.2 207 3.6 231.1 0 256 0s49 3.6 71.8 10.2C320.9 15.8 315 23 310.6 31.8L300.3 52.7C286 49.6 271.2 48 256 48zM372.2 302.3c11.8-3.6 23.7 6.1 19.6 17.8-19.8 55.9-73.1 96-135.8 96-62.7 0-116-40-135.8-95.9-4.1-11.6 7.8-21.4 19.6-17.8 34.7 10.6 74.2 16.5 116.1 16.5 42 0 81.5-6 116.3-16.6zM353.7 53.1c5.9-11.9 22.8-11.9 28.7 0l23.3 47.2 52 7.6c13.1 1.9 18.4 18 8.9 27.3l-37.7 36.7 8.9 51.8c2.2 13.1-11.5 23-23.2 16.9L368 216 321.5 240.5c-11.7 6.2-25.5-3.8-23.2-16.9l8.9-51.8-37.7-36.7c-9.5-9.3-4.3-25.4 8.9-27.3l52-7.6 23.3-47.2zm-195.3 0l23.3 47.2 52 7.6c13.1 1.9 18.4 18 8.9 27.3l-37.7 36.7 8.9 51.8c2.2 13.1-11.5 23-23.2 16.9L144 216 97.5 240.5c-11.7 6.2-25.5-3.8-23.2-16.9l8.9-51.8-37.7-36.7c-9.5-9.3-4.3-25.4 8.9-27.3l52-7.6 23.3-47.2c5.9-11.9 22.8-11.9 28.7 0z",
            }
        }
    }
}
