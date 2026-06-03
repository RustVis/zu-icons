// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ChessPawnProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn ChessPawn(props: ChessPawnProps) -> Element {
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
        d: "M192-32c66.3 0 120 53.7 120 120 0 27.6-9.3 52.9-24.9 73.2 9.8 3 16.9 12.1 16.9 22.8 0 13.3-10.7 24-24 24l-.6 0 24.6 160 53.6 67c6.7 8.4 10.4 18.8 10.4 29.6 0 26.2-21.2 47.4-47.4 47.4L63.4 512c-26.2 0-47.4-21.2-47.4-47.4 0-10.8 3.7-21.2 10.4-29.6l53.6-67 24.6-160-.6 0c-13.3 0-24-10.7-24-24 0-10.8 7.1-19.8 16.9-22.8-15.6-20.3-24.9-45.6-24.9-73.2 0-66.3 53.7-120 120-120zM115.9 400l-51.2 64 254.7 0-51.2-64-152.2 0zm36.2-184.7l-21 136.7 121.9 0-21-136.7-1.1-7.3-77.6 0-1.1 7.3zM192 16a72 72 0 1 0 0 144 72 72 0 1 0 0-144z",
            }
        }
    }
}
