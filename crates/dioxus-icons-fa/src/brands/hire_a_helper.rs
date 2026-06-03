// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HireAHelperProps {
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

pub fn HireAHelper(props: HireAHelperProps) -> Element {
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
        d: "M443.1 0L71.9 0C67.9 37.3 37.4 67.8 0 71.7L0 443.2c37.4 4.9 66 32.4 71.9 68.8l372.2 0c3-36.4 32.5-65.8 67.9-69.8l0-370.5C475.6 65.8 447 36.4 443.1 0zm-37 404.9c-36.3 0-18.8-2-55.1-2-35.8 0-21 2-56.1 2-5.9 0-4.9-8.2 0-9.8 22.8-7.6 22.9-10.2 24.6-12.8 10.4-15.6 5.9-83 5.9-113 0-5.3-6.4-12.8-13.8-12.8l-111.2 0c-7.4 0-13.8 7.5-13.8 12.8 0 30-4.5 97.4 5.9 113 1.7 2.5 1.8 5.2 24.6 12.8 4.9 1.6 6 9.8 0 9.8-35.1 0-20.3-2-56.1-2-36.3 0-18.8 2-55.1 2-7.9 0-5.8-10.8 0-10.8 10.2-3.4 13.5-3.5 21.7-13.8 7.7-12.9 7.9-44.4 7.9-127.8l0-101.2c0-22.2-12.2-28.3-28.6-32.4-8.8-2.2-4-11.8 1-11.8 36.5 0 20.6 2 57.1 2 32.7 0 16.5-2 49.2-2 3.3 0 8.5 8.3 1 10.8-4.9 1.6-27.6 3.7-27.6 39.3 0 45.6-.2 55.8 1 68.8 0 1.3 2.3 12.8 12.8 12.8l109.2 0c10.5 0 12.8-11.5 12.8-12.8 1.2-13 1-23.2 1-68.8 0-35.6-22.7-37.7-27.6-39.3-7.5-2.5-2.3-10.8 1-10.8 32.7 0 16.5 2 49.2 2 36.5 0 20.6-2 57.1-2 4.9 0 9.9 9.6 1 11.8-16.4 4.1-28.6 10.3-28.6 32.4l0 101.2c0 83.4 .1 114.9 7.9 127.8 8.2 10.2 11.4 10.4 21.7 13.8 5.8 0 7.8 10.8 0 10.8z",
            }
        }
    }
}
