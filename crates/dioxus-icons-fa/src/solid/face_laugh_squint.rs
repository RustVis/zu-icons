// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FaceLaughSquintProps {
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

pub fn FaceLaughSquint(props: FaceLaughSquintProps) -> Element {
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
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zM107.7 332.1C102 318 113.4 304 128.6 304l254.9 0c15.2 0 26.6 14 20.9 28.1-23.8 58.6-81.2 99.9-148.3 99.9s-124.6-41.3-148.3-99.9zm15-188.8c4.5-6.8 13.3-9.2 20.6-5.5l79.6 40c5.4 2.7 8.8 8.2 8.8 14.3s-3.4 11.6-8.8 14.3l-79.6 40c-7.3 3.6-16.1 1.3-20.6-5.5s-3.1-15.9 3.1-21.1L159 192 125.8 164.3c-6.2-5.2-7.6-14.3-3.1-21.1zm263.6 21.1L353 192 386.2 219.7c6.2 5.2 7.6 14.3 3.1 21.1s-13.3 9.2-20.6 5.5l-79.6-40c-5.4-2.7-8.8-8.2-8.8-14.3s3.4-11.6 8.8-14.3l79.6-40c7.3-3.6 16.1-1.3 20.6 5.5s3.1 15.9-3.1 21.1z",
            }
        }
    }
}
