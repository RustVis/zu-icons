// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AngryProps {
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

pub fn Angry(props: AngryProps) -> Element {
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
        d: "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464a256 256 0 1 0 0-512 256 256 0 1 0 0 512zm0-144c24.1 0 45.4 11.8 58.5 30 7.7 10.8 22.7 13.2 33.5 5.5s13.2-22.7 5.5-33.5c-21.7-30.2-57.3-50-97.5-50s-75.7 19.8-97.5 50c-7.7 10.8-5.3 25.8 5.5 33.5s25.8 5.3 33.5-5.5c13.1-18.2 34.4-30 58.5-30zm-80-96c17.7 0 32-14.3 32-32l0-.3 9.7 3.2c10.5 3.5 21.8-2.2 25.3-12.6s-2.2-21.8-12.6-25.3l-96-32c-10.5-3.5-21.8 2.2-25.3 12.6s2.2 21.8 12.6 25.3l28.9 9.6c-4.1 5.4-6.6 12.1-6.6 19.4 0 17.7 14.3 32 32 32zm192-32c0-7.3-2.4-14-6.6-19.4l28.9-9.6c10.5-3.5 16.1-14.8 12.6-25.3s-14.8-16.1-25.3-12.6l-96 32c-10.5 3.5-16.1 14.8-12.6 25.3s14.8 16.1 25.3 12.6l9.7-3.2 0 .3c0 17.7 14.3 32 32 32s32-14.3 32-32z",
            }
        }
    }
}
