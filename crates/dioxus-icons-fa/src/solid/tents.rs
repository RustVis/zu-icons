// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TentsProps {
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

pub fn Tents(props: TentsProps) -> Element {
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
        d: "M539.9 352l-95.6 0-8.4-66.1c-2.9-23.2-15.9-43.9-35.4-56.8L252.2 131.5c-18-11.9-39.7-15.7-60.1-11.5 2.5-4.3 5.9-7.9 10.1-10.7L350.6 11.6c10.7-7 24.5-7 35.2 0l148.4 97.7c7.8 5.1 13 13.4 14.2 22.7l23.3 184c2.4 19.1-12.5 36-31.7 36zM4.8 476l23.3-184c1.2-9.3 6.3-17.6 14.2-22.7l148.4-97.7c10.7-7 24.5-7 35.2 0l148.4 97.7c7.8 5.1 13 13.4 14.2 22.7l23.3 184c2.4 19.1-12.5 36-31.7 36l-49.7 0c-11.2 0-21.5-5.8-27.3-15.4l-77-126.7c-1.7-2.8-4.8-4.6-8.1-4.6-5.3 0-9.5 4.3-9.5 9.5l0 105.2c0 17.7-14.3 32-32 32L36.5 512c-19.3 0-34.2-16.9-31.7-36z",
            }
        }
    }
}
