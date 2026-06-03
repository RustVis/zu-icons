// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PerbyteProps {
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

pub fn Perbyte(props: PerbyteProps) -> Element {
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
        d: "M305.3 284.6l-58.7 0 0 98.7 58.7 0c16.3 0 29-4.6 38.2-13.8s13.8-21.2 13.8-36.1c0-14.6-4.7-26.3-14-35.3s-22-13.5-37.9-13.5zM149.4 128.7l-58.7 0 0 98.7 58.7 0c16.3 0 29-4.6 38.2-13.8s13.8-21.2 13.8-36.1c0-14.6-4.7-26.3-14-35.3s-22-13.5-37.9-13.5zM366.6 32L81.4 32c-21.6 0-42.2 8.6-57.5 23.9S0 91.8 0 113.4L0 398.6c0 21.6 8.6 42.2 23.9 57.5S59.8 480 81.4 480l285.3 0c21.6 0 42.2-8.6 57.5-23.9S448 420.2 448 398.6l0-285.3c0-21.6-8.6-42.2-23.9-57.5S388.2 32 366.6 32zm63.6 366.6c0 16.9-6.7 33-18.7 45s-28.1 18.6-45 18.7l-285.3 0c-16.9 0-33-6.7-45-18.7s-18.6-28.1-18.7-45l0-285.3c0-16.9 6.7-33 18.7-45s28.1-18.6 45-18.7l285.3 0c16.9 0 33 6.7 45 18.7s18.6 28.1 18.7 45l0 285.3zm-125-269.9l-58.7 0 0 98.7 58.7 0c16.3 0 29-4.6 38.2-13.8s13.8-21.2 13.8-36.1c0-14.6-4.7-26.3-14-35.3s-22-13.5-37.9-13.5z",
            }
        }
    }
}
