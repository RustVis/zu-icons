// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VnvProps {
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

pub fn Vnv(props: VnvProps) -> Element {
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
        d: "M104.9 352c-34.1 0-46.4-30.4-46.4-30.4L2.6 210.1S-7.8 192 13 192l32.8 0c10.4 0 13.2 8.7 18.8 18.1l36.7 74.5s5.2 13.1 21.1 13.1 21.1-13.1 21.1-13.1l36.7-74.5c5.6-9.5 8.4-18.1 18.8-18.1l32.8 0c20.8 0 10.4 18.1 10.4 18.1L186.4 321.6S174.2 352 140 352l-35.1 0zm395 0c-34.1 0-46.4-30.4-46.4-30.4L397.6 210.1S387.2 192 408 192l32.8 0c10.4 0 13.2 8.7 18.8 18.1l36.7 74.5s5.2 13.1 21.1 13.1 21.1-13.1 21.1-13.1l36.8-74.5c5.6-9.5 8.4-18.1 18.8-18.1l32.9 0c20.8 0 10.4 18.1 10.4 18.1L581.5 321.6S569.3 352 535.1 352l-35.2 0zM337.6 192c34.1 0 46.4 30.4 46.4 30.4l55.9 111.5S450.3 352 429.5 352l-32.8 0c-10.4 0-13.2-8.7-18.8-18.1l-36.7-74.5s-5.2-13.1-21.1-13.1-21.1 13.1-21.1 13.1l-36.7 74.5c-5.6 9.4-8.4 18.1-18.8 18.1l-32.9 0c-20.8 0-10.4-18.1-10.4-18.1l55.9-111.5S268.3 192 302.5 192l35.1 0z",
            }
        }
    }
}
