// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BaseballBallProps {
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

pub fn BaseballBall(props: BaseballBallProps) -> Element {
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
        d: "M232.4 1.1c-122.2 11.4-219.5 108.7-230.9 230.9 16.2-.1 32-1.8 47.2-5 13-2.7 25.7 5.5 28.4 18.5s-5.5 25.7-18.5 28.4c-18.4 3.9-37.6 6-57.1 6.1 11.4 122.2 108.7 219.5 230.9 230.9 .1-19.5 2.2-38.7 6.1-57.1 2.7-13 15.5-21.3 28.4-18.5s21.3 15.5 18.5 28.4c-3.2 15.2-4.9 31-5 47.2 122.2-11.4 219.5-108.7 230.9-230.9-16.2 .1-32 1.8-47.2 5-13 2.7-25.7-5.5-28.4-18.5s5.5-25.7 18.5-28.4c18.4-3.9 37.6-6 57.1-6.1-11.4-122.2-108.7-219.5-230.9-230.9-.1 19.5-2.2 38.7-6.1 57.1-2.7 13-15.5 21.3-28.4 18.5s-21.3-15.5-18.5-28.4c3.2-15.2 4.9-31 5-47.2zm2.8 151.4c-21.4 32.9-49.5 60.9-82.3 82.3-11.1 7.2-26 4.1-33.2-7s-4.1-26 7-33.2c27.2-17.7 50.5-41 68.3-68.3 7.2-11.1 22.1-14.3 33.2-7s14.3 22.1 7 33.2zM393.1 284.2c7.2 11.1 4.1 26-7 33.2-27.2 17.7-50.5 41-68.3 68.3-7.2 11.1-22.1 14.3-33.2 7s-14.3-22.1-7-33.2c21.4-32.9 49.5-60.9 82.3-82.3 11.1-7.2 26-4.1 33.2 7z",
            }
        }
    }
}
