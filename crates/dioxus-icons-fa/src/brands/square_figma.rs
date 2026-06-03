// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SquareFigmaProps {
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

pub fn SquareFigma(props: SquareFigmaProps) -> Element {
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
        d: "M384 32c35.3 0 64 28.7 64 64l0 320c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64L0 96C0 60.7 28.7 32 64 32l320 0zM173.7 96c-33 0-59.8 26.8-59.8 59.8 0 21 10.8 39.4 27.2 50.1-16.4 10.7-27.2 29.1-27.2 50.1s10.8 39.5 27.2 50.1c-16.4 10.7-27.2 29.1-27.2 50.1 0 33.1 27.1 59.8 60.1 59.8 33.2 0 60.6-26.9 60.6-60.3l0-55.7c10.6 9.8 24.8 15.8 40.4 15.8l1.1 0c33 0 59.8-26.8 59.8-59.8 0-21-10.8-39.5-27.2-50.1 16.4-10.7 27.2-29.1 27.2-50.1 0-33-26.8-59.8-59.8-59.8L173.7 96zm41.6 219.8l0 39.9c0 22.6-18.6 41-41.3 41-22.4 0-40.7-18.2-40.7-40.5 0-22.3 18.1-40.4 40.3-40.4l41.7 0zm0-100.3l0 80.9-41.7 0c-22.3-.1-40.3-18.2-40.3-40.5 0-22.3 18.1-40.5 40.5-40.5l41.6 0zm60.8 0c22.3 0 40.5 18.1 40.5 40.5s-18.1 40.5-40.5 40.5l-1.1 0c-22.3 0-40.4-18.1-40.4-40.5s18.1-40.5 40.4-40.5l1.1 0zm-60.8-19.3l-41.6 0c-22.3 0-40.5-18.1-40.5-40.4s18.1-40.5 40.5-40.5l41.6 0 0 80.9zm60.8-80.9c22.3 0 40.5 18.1 40.5 40.5s-18.1 40.4-40.5 40.4l-41.5 0 0-80.9 41.5 0z",
            }
        }
    }
}
