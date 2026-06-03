// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BootstrapProps {
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

pub fn Bootstrap(props: BootstrapProps) -> Element {
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
        d: "M333.5 201.4c0-22.1-15.6-34.3-43-34.3l-50.4 0 0 71.2 42.5 0c32.8-.1 50.9-13.3 50.9-36.9zM517 188.6c-9.5-30.9-10.9-68.8-9.8-98.1 1.1-30.5-22.7-58.5-54.7-58.5L123.7 32c-32.1 0-55.8 28.1-54.7 58.5 1 29.3-.3 67.2-9.8 98.1-9.6 31-25.7 50.6-52.2 53.1l0 28.5c26.4 2.5 42.6 22.1 52.2 53.1 9.5 30.9 10.9 68.8 9.8 98.1-1.1 30.5 22.7 58.5 54.7 58.5l328.7 0c32.1 0 55.8-28.1 54.7-58.5-1-29.3 .3-67.2 9.8-98.1 9.6-31 25.7-50.6 52.1-53.1l0-28.5c-26.3-2.5-42.5-22.1-52-53.1zM300.2 375.1l-97.9 0 0-238.3 97.4 0c43.3 0 71.7 23.4 71.7 59.4 0 25.3-19.1 47.9-43.5 51.8l0 1.3c33.2 3.6 55.5 26.6 55.5 58.3 0 42.1-31.3 67.5-83.2 67.5zm-10-108.7l-50.1 0 0 78.4 52.3 0c34.2 0 52.3-13.7 52.3-39.5 0-25.7-18.6-38.9-54.5-38.9z",
            }
        }
    }
}
