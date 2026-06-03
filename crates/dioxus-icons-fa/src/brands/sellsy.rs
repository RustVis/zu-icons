// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SellsyProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Sellsy(props: SellsyProps) -> Element {
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
        d: "M540 237.3c3.1-12.3 4.3-24.8 4.3-37.4 0-92.5-75.4-167.9-167.9-167.9-77.2 0-144.6 53-163 127.8-15.3-13.2-34.9-20.5-55.2-20.5-46.3 0-84 37.7-84 84 0 7.4 .9 15 3.1 22.4-42.9 20.2-70.8 63.7-70.8 111.2 0 68 55.5 123.2 123.2 123.2l381.2 0c67.7 0 123.2-55.2 123.2-123.2 0-56.4-38.9-106-94.1-119.5zM200.2 401.6c0 8.3-7 15.3-15.3 15.3l-30.9 0c-8.3 0-15.3-7-15.3-15.3l0-110.9c0-8.3 7-15.3 15.3-15.3l30.9 0c8.3 0 15.3 7 15.3 15.3l0 110.9zm89.5 0c0 8.3-7 15.3-15.3 15.3l-30.9 0c-8.3 0-15.3-7-15.3-15.3l0-131.5c0-8.3 7-15.3 15.3-15.3l30.9 0c8.3 0 15.3 7 15.3 15.3l0 131.5zm89.5 0c0 8.3-7 15.3-15.3 15.3l-31 0c-8.3 0-15.3-7-15.3-15.3l0-162.7c0-8.3 7-15.3 15.3-15.3l31 0c8.3 0 15.3 7 15.3 15.3l0 162.7zm87 0c0 8.3-7 15.3-15.3 15.3l-28.5 0c-8.3 0-15.3-7-15.3-15.3l0-224.6c0-8.6 7-15.6 15.3-15.6l28.5 0c8.3 0 15.3 7 15.3 15.6l0 224.6z",
            }
        }
    }
}
