// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct StackpathProps {
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

pub fn Stackpath(props: StackpathProps) -> Element {
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
        d: "M244.6 232.4c0 8.5-4.3 20.5-21.3 20.5l-19.6 0 0-41.5 19.6 0c17.1 0 21.3 12.4 21.3 21zM448 32l0 448-448 0 0-448 448 0zM151.3 287.8c0-21.2-12.1-34.5-46.7-44.8-20.6-7.4-26-10.9-26-18.6s7-14.6 20.4-14.6c14.1 0 20.8 8.4 20.8 18.4l30.7 0 .2-.6c.5-19.6-15.1-41.6-51.1-41.6-23.4 0-52.6 10.8-52.6 38.3 0 19.4 9.2 31.3 50.7 44.4 17.3 6.2 21.9 10.4 21.9 19.5 0 15.2-19.1 14.2-19.5 14.2-20.4 0-25.7-9.1-25.7-21.9l-30.8 0-.2 .6c-.7 31.3 28.4 45.2 56.6 45.2 30 0 51.1-13.6 51.1-38.3zm125.4-55.6c0-25.3-18.4-45.5-53.4-45.5l-51.8 0 0 138.2 32.2 0 0-47.4 19.6 0c30.3 0 53.4-16 53.4-45.4zM297.9 325l49.1-138.2-31.1 0-47.9 138.2 29.9 0zM404.5 186.8l-31.1 0-47.9 138.2 29.9 0 49.1-138.2z",
            }
        }
    }
}
